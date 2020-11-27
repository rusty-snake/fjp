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
    profile::{Error as ProfileError, Profile, ProfileFlags},
};
use anyhow::{bail, ensure};
use clap::ArgMatches;
use log::debug;
use std::fmt::Write;

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
        ProfileFlags::default().with(ProfileFlags::READ),
    )
    .unwrap_or_else(|err| {
        if let ProfileError::ReadError {
            full_name, source, ..
        } = err
        {
            fatal!("Failed to read {}: {}", full_name, source)
        }
        unreachable!();
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
        if let Some(other_profile) = line.strip_prefix("include ") {
            if keep_inc && line.ends_with(".inc") {
                write_lined!(line, standalone_profile);
            } else {
                ensure!(recusion_level <= 16, "To many include levels");
                match Profile::new(
                    other_profile,
                    ProfileFlags::default().with(ProfileFlags::READ),
                ) {
                    Ok(profile) => process(
                        profile.raw_data().unwrap(),
                        standalone_profile,
                        recusion_level + 1,
                        keep_inc,
                    )?,
                    Err(err) => {
                        if let ProfileError::ReadError { ref source, .. } = err {
                            if let Some(ProfileError::NoPath) =
                                source.downcast_ref::<ProfileError>()
                            {
                                return Ok(());
                            } else {
                                bail!(err);
                            }
                        } else {
                            bail!(err);
                        }
                    }
                }
            }
        } else {
            write_lined!(line, standalone_profile);
        }
    }
    Ok(())
}
