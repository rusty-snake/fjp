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
    profile::{ErrorContext as NewProfileErrorContext, NewProfileFlags, Profile},
};
use anyhow::{bail, ensure};
use clap::ArgMatches;
use log::debug;
use std::fmt::Write;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

macro_rules! write_lined {
    ($src:expr, $dst:ident) => {
        $dst.write_str($src).unwrap();
        $dst.write_char('\n').unwrap();
    };
}

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: generate-standalone");

    let keep_inc = cli.is_present("keep-inc");

    let mut standalone_profile = String::new();

    let profile = Profile::new(
        cli.value_of("PROFILE_NAME").unwrap(),
        NewProfileFlags::default_with(NewProfileFlags::READ),
    )
    .unwrap_or_else(|err| {
        if let Some(err_ctx) = err.downcast_ref::<NewProfileErrorContext>() {
            if let Some(io_err) = err.downcast_ref::<IoError>() {
                fatal!("Failed to read {}: {}", err_ctx.full_name, io_err);
            }
        }
        fatal!("{}", err)
    });

    let res = process(
        profile.raw_data().unwrap(),
        &mut standalone_profile,
        0,
        keep_inc,
    );
    if let Err(err) = res {
        fatal!("{}", err);
    }

    println!("{}", standalone_profile);
}

fn process(
    data: &str,
    standalone_profile: &mut String,
    recusion_level: u8,
    keep_inc: bool,
) -> anyhow::Result<()> {
    for line in data.lines() {
        if line.starts_with("include ") {
            if keep_inc && line.ends_with(".inc") {
                write_lined!(line, standalone_profile);
            } else {
                ensure!(recusion_level <= 16, "To many include levels");
                match Profile::new(
                    &line[8..],
                    NewProfileFlags::default_with(NewProfileFlags::READ),
                ) {
                    Ok(profile) => process(
                        profile.raw_data().unwrap(),
                        standalone_profile,
                        recusion_level + 1,
                        keep_inc,
                    )?,
                    Err(err) => match err.downcast::<IoError>() {
                        Ok(err) => {
                            if err.kind() == IoErrorKind::NotFound {
                                return Ok(());
                            } else {
                                bail!(err);
                            }
                        }
                        Err(err) => bail!(err),
                    },
                }
            }
        } else {
            write_lined!(line, standalone_profile);
        }
    }
    Ok(())
}
