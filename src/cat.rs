/*
 * Copyright © 2020,2021 The fjp Authors
 *
 * This file is part of fjp
 *
 * fjp is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * fjp is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::profile::{Profile, ProfileFlags};
use crate::{fatal, utils::ColoredText};
use clap::ArgMatches;
use log::{debug, error, warn};
use nix::sys::signal::{kill, Signal::SIGTERM};
use nix::unistd::Pid;
use std::convert::TryInto;
use std::io;
use std::process::{Child, Command, Stdio};
use termcolor::Color;

#[derive(Debug, Default)]
struct Options {
    show_locals: bool,
    show_redirects: bool,
}

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: cat");

    let cmd: &[&str] = if cli.is_present("no-pager") {
        &["cat"]
    } else {
        &["less", "-R"]
    };

    let mut child: Option<Child> = Command::new(cmd[0])
        .args(&cmd[1..])
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_or_else(
            |err| {
                warn!("Failed to start {}: {}", cmd[0], err);
                warn!("Continue without it.");
                None
            },
            Some,
        );

    let opts = Options {
        show_locals: !cli.is_present("no-locals"),
        show_redirects: !cli.is_present("no-redirects"),
    };
    let name = cli.value_of("PROFILE_NAME").unwrap();
    let profile_flags = ProfileFlags::default().with(ProfileFlags::READ);

    match Profile::new(name, profile_flags) {
        Ok(p) => {
            let mut output: Box<dyn io::Write> = if let Some(ref mut child) = child {
                Box::new(child.stdin.as_mut().unwrap())
            } else {
                Box::new(io::stdout())
            };
            process(&p, p.raw_data(), &opts, &mut output, 0);
        }
        Err(e) => {
            if let Some(ref child) = child {
                kill(Pid::from_raw(child.id().try_into().unwrap()), SIGTERM).unwrap();
            }
            error!("Couldn't Read Profile. {}", e);
        }
    };

    if let Some(ref mut child) = child {
        child.wait().unwrap();
    }
}

fn process<W: io::Write>(
    profile: &Profile<'_>,
    content: &str,
    opts: &Options,
    output: &mut W,
    mut depth: u8,
) {
    if depth >= 16 {
        fatal!("To many include levels");
    }
    depth += 1;

    let [locals, profiles] = parse(content);

    if opts.show_locals {
        if let Some(locals) = locals {
            show_locals(&locals, opts, output);
        }
    }

    show_file(profile, content, output);

    if opts.show_redirects {
        if let Some(profiles) = profiles {
            show_profiles(&profiles, opts, output, depth);
        }
    }
}

fn parse(content: &str) -> [Option<Vec<String>>; 2] {
    let mut local = Vec::new();
    let mut profile = Vec::new();

    for line in content.lines() {
        if let Some(other_profile) = line.strip_prefix("include ") {
            if other_profile.ends_with(".local") {
                local.push(other_profile.to_string());
            } else if other_profile.ends_with(".profile") {
                profile.push(other_profile.to_string());
            }
        }
    }

    [
        if local.is_empty() { None } else { Some(local) },
        if profile.is_empty() {
            None
        } else {
            Some(profile)
        },
    ]
}

fn show_file<W: io::Write>(profile: &Profile<'_>, content: &str, output: &mut W) {
    output
        .write_all(
            ColoredText::new(
                Color::Blue,
                &format!("# {}:\n", profile.path().unwrap().to_string_lossy()),
            )
            .as_bytes(),
        )
        .unwrap();
    output.write_all(content.as_bytes()).unwrap();
}

fn show_locals<W: io::Write>(locals: &[String], _opts: &Options, output: &mut W) {
    locals
        .iter()
        .filter(|&name| {
            name != "globals.local" && name != "pre-globals.local" && name != "post-globals.local"
        })
        .filter_map(|name| {
            Profile::new(name, ProfileFlags::default().with(ProfileFlags::READ)).ok()
        })
        .for_each(|profile| {
            show_file(&profile, profile.raw_data(), output);
        });
}

fn show_profiles<W: io::Write>(profiles: &[String], opts: &Options, output: &mut W, depth: u8) {
    for name in profiles {
        let profile_flags = ProfileFlags::default().with(ProfileFlags::READ);
        match Profile::new(name, profile_flags) {
            Ok(p) => {
                process(&p, p.raw_data(), opts, output, depth);
            }
            Err(e) => {
                error!("Couldn't Read profile. {}", e);
            }
        };
    }
}
