/*
 * Copyright © 2020,2021 rusty-snake
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

use crate::{fatal, USER_PROFILE_DIR};
use bitflags::bitflags;
use clap::ArgMatches;
use log::{debug, warn};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;

bitflags! {
    struct Flags: u8 {
        const INCS     = 0b_0000_0001;
        const LOCALS   = 0b_0000_0010;
        const PROFILES = 0b_0000_0100;
    }
}
impl Flags {
    fn from_cli_args(cli: &ArgMatches<'_>) -> Self {
        macro_rules! insert_flag {
            ($flag:path, in $flags:ident if $cond:expr) => {
                if $cond {
                    $flags.insert($flag);
                }
            };
        }

        let mut flags = Self::empty();
        insert_flag!(Self::INCS, in flags if cli.is_present("incs"));
        insert_flag!(Self::LOCALS, in flags if cli.is_present("locals"));
        insert_flag!(Self::PROFILES, in flags if cli.is_present("profiles"));
        flags
    }
}

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: list");

    let flags = Flags::from_cli_args(cli);

    let mut user_profiles = read_dir(&*USER_PROFILE_DIR)
        .unwrap_or_else(|err| fatal!("Failed to open the user profile directory: {}", err))
        .filter_map(|readdir_result| match readdir_result {
            Ok(direntry) => Some(direntry),
            Err(e) => {
                warn!("{}", e);
                None
            }
        })
        .filter(|direntry| direntry.file_type().unwrap().is_file())
        .map(|file| file.file_name())
        .filter(|file| {
            flags.contains(Flags::INCS) && Path::new(file).extension() == Some(OsStr::new(".inc"))
        })
        .filter(|file| {
            flags.contains(Flags::LOCALS)
                && Path::new(file).extension() == Some(OsStr::new(".local"))
        })
        .filter(|file| {
            flags.contains(Flags::PROFILES)
                && Path::new(file).extension() == Some(OsStr::new(".profile"))
        })
        .collect::<Vec<_>>();
    user_profiles.sort_unstable();
    print!(
        "{}",
        user_profiles
            .into_iter()
            .fold(String::new(), |mut acc, val| {
                acc.push_str(val.to_str().unwrap());
                acc.push('\n');
                acc
            })
    );
}
