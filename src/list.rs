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

use crate::{fatal, USER_PROFILE_DIR};
use bitflags::bitflags;
use clap::ArgMatches;
use log::{debug, warn};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::{stdout, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

bitflags! {
    struct Flags: u8 {
        const ONLY_INCS     = 0b_0000_0001;
        const ONLY_LOCALS   = 0b_0000_0010;
        const ONLY_PROFILES = 0b_0000_0100;
    }
}
impl Flags {
    fn from_cli_args(cli: &ArgMatches<'_>) -> Self {
        macro_rules! flags_from_cli_args {
            ( $( $flag:literal: $galf:ident $(,)? )* ) => {
                let mut flags = Self::empty();
                $(
                    if cli.is_present($flag) {
                        flags.insert(Self::$galf)
                    }
                )*
                return flags;
            };
        }

        flags_from_cli_args! {
            "incs": ONLY_INCS,
            "locals": ONLY_LOCALS,
            "profiles": ONLY_PROFILES,
        }
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
            !flags.contains(Flags::ONLY_INCS)
                || Path::new(file).extension() == Some(OsStr::new("inc"))
        })
        .filter(|file| {
            !flags.contains(Flags::ONLY_LOCALS)
                || Path::new(file).extension() == Some(OsStr::new("local"))
        })
        .filter(|file| {
            !flags.contains(Flags::ONLY_PROFILES)
                || Path::new(file).extension() == Some(OsStr::new("profile"))
        })
        .collect::<Vec<_>>();
    user_profiles.sort_unstable();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    for user_profile in user_profiles {
        stdout.write_all(user_profile.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}
