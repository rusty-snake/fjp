/*
 * Copyright © 2020 rusty-snake
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

use crate::{
    fatal,
    utils::{find_profile, get_name1, ColoredText},
};
use clap::ArgMatches;
use log::{debug, error, warn};
use std::fs::read_to_string;
use std::io;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use termcolor::Color;

#[derive(Debug, Default)]
struct Options {
    show_globals: bool,
    show_locals: bool,
    show_redirects: bool,
}

#[derive(Debug)]
struct Profile {
    path: PathBuf,
    data: String,
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
        .spawn()
        .map_or_else(
            |err| {
                warn!("Failed to start {}: {}", cmd[0], err);
                warn!("Continue without it.");
                None
            },
            Some,
        );
    let mut output: Box<dyn io::Write> = if let Some(ref mut child) = child {
        Box::new(child.stdin.as_mut().unwrap())
    } else {
        Box::new(io::stdout())
    };

    let name = get_name1(cli.value_of("PROFILE_NAME").unwrap());
    let path = find_profile(&name).unwrap_or_else(|| fatal!("Can not find {}.", &name));
    let data = read_to_string(&path)
        .unwrap_or_else(|e| fatal!("Failed to open {}: {}", path.to_string_lossy(), e));

    let profile = Profile { path, data };

    let opts = Options {
        show_globals: !cli.is_present("no-globals"),
        show_locals: !cli.is_present("no-locals"),
        show_redirects: !cli.is_present("no-redirects"),
    };

    process(&profile, &opts, &mut output, 0);

    drop(output); // We need to drop output here, otherwise we would have two mutable references.
    if let Some(ref mut child) = child {
        child.wait().unwrap();
    }
}

fn process<W: io::Write>(profile: &Profile, opts: &Options, output: &mut W, mut depth: u8) {
    if depth >= 16 {
        fatal!("To many include levels");
    }
    depth += 1;

    let [locals, profiles] = parse(&profile.data);

    if opts.show_locals {
        if let Some(locals) = locals {
            show_locals(&locals, opts, output);
        }
    }

    show_file(profile, output);

    if opts.show_redirects {
        if let Some(profiles) = profiles {
            show_profiles(&profiles, opts, output, depth);
        }
    }
}

fn parse(data: &str) -> [Option<Vec<String>>; 2] {
    let mut local = Vec::new();
    let mut profile = Vec::new();

    for line in data.lines() {
        if line.starts_with("include ") {
            if line.ends_with(".local") {
                local.push(unsafe { line.get_unchecked(8..) }.to_string());
            } else if line.ends_with(".profile") {
                profile.push(unsafe { line.get_unchecked(8..) }.to_string());
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

fn show_file<W: io::Write>(profile: &Profile, output: &mut W) {
    output
        .write_all(
            ColoredText::new(
                Color::Blue,
                &format!("# {}:\n", profile.path.to_string_lossy()),
            )
            .as_bytes(),
        )
        .unwrap();
    output.write_all(profile.data.as_bytes()).unwrap();
}

fn show_locals<W: io::Write>(locals: &[String], opts: &Options, output: &mut W) {
    locals
        .iter()
        .filter(|name| opts.show_globals || *name != "globals.local")
        .filter_map(|name| match find_profile(name) {
            Some(path) => Some(path),
            None => {
                warn!("{} could not be found.", name);
                None
            }
        })
        .filter_map(|path| match read_to_string(&path) {
            Ok(data) => Some(Profile { path, data }),
            Err(err) => {
                error!("Failed to open {}: {}", path.to_string_lossy(), err);
                None
            }
        })
        .for_each(|profile| show_file(&profile, output));
}

fn show_profiles<W: io::Write>(profiles: &[String], opts: &Options, output: &mut W, depth: u8) {
    for name in profiles {
        let path = match find_profile(name) {
            Some(path) => path,
            None => {
                error!("Can not find {}.", name);
                continue;
            }
        };

        let data = match read_to_string(&path) {
            Ok(data) => data,
            Err(err) => {
                error!("Failed to read {}: {}", path.to_string_lossy(), err);
                continue;
            }
        };

        process(&Profile { path, data }, opts, output, depth);
    }
}
