/*
 * Copyright © 2020-2022 The fjp Authors
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
use anyhow::{anyhow, ensure};
use bitflags::bitflags;
use clap::ArgMatches;
use log::debug;
use std::error::Error as StdError;
use std::fs::File;
use std::io::{stdout, BufWriter, Write as IoWrite};

bitflags! {
    struct Flags: u8 {
        const KEEP_INCS     = 0b_0000_0001;
        const KEEP_LOCALS   = 0b_0000_0010;
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RecusionLevel(u8);
impl RecusionLevel {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn max() -> Self {
        Self(16)
    }

    pub const fn incremented(current: Self) -> Self {
        Self(current.0 + 1)
    }
}

macro_rules! write_lined {
    ($src:expr, $dst:ident) => {
        $dst.write_all($src.as_bytes()).unwrap();
        $dst.write_all(b"\n").unwrap();
    };
}

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: generate-standalone");

    let mut flags = Flags::empty();
    if cli.is_present("keep-incs") {
        flags.insert(Flags::KEEP_INCS);
    }
    if cli.is_present("keep-locals") {
        flags.insert(Flags::KEEP_LOCALS);
    }

    let mut output: BufWriter<Box<dyn IoWrite>> =
        BufWriter::new(cli.value_of("output").map_or_else(
            || Box::new(stdout()) as Box<dyn IoWrite>,
            |file_name| {
                Box::new(
                    File::create(file_name)
                        .unwrap_or_else(|err| fatal!("Failed to create output file: {}", err)),
                )
            },
        ));

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

    process(&profile, &mut output, RecusionLevel::zero(), flags)
        .unwrap_or_else(|err| fatal!("{}", err));

    output.flush().unwrap();
}

fn process(
    profile: &Profile<'_>,
    output: &mut dyn IoWrite,
    recusion_level: RecusionLevel,
    flags: Flags,
) -> anyhow::Result<()> {
    writeln!(output, "## Begin {} ##", profile.full_name()).unwrap();
    for line in profile.raw_data().lines() {
        if let Some(other_profile) = line.strip_prefix("include ") {
            if (flags.contains(Flags::KEEP_INCS) && line.ends_with(".inc"))
                || (flags.contains(Flags::KEEP_LOCALS) && line.ends_with(".local"))
            {
                write_lined!(line, output);
            } else {
                ensure!(
                    recusion_level <= RecusionLevel::max(),
                    "To many include levels"
                );

                match Profile::new(
                    other_profile,
                    ProfileFlags::default().with(ProfileFlags::READ),
                ) {
                    Ok(profile) => process(
                        &profile,
                        output,
                        RecusionLevel::incremented(recusion_level),
                        flags,
                    ),
                    Err(err) if caused_by_no_path(&err) => Ok(()),
                    Err(err) => Err(anyhow!("Failed to read '{}': {}", other_profile, err)),
                }?;
            }
        } else {
            write_lined!(line, output);
        }
    }
    writeln!(output, "## End {} ##", profile.full_name()).unwrap();
    Ok(())
}

fn caused_by_no_path(err: &(dyn StdError + 'static)) -> bool {
    if let Some(ProfileError::NoPath) = err.downcast_ref() {
        true
    } else if let Some(e) = err.source() {
        caused_by_no_path(e)
    } else {
        false
    }
}
