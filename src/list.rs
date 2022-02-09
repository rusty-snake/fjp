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

use crate::{fatal, USER_PROFILE_DIR};
use log::{debug, warn};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::io::{stdout, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub fn start(cli: &crate::cli::CliList) {
    debug!("subcommand: list");

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
        .filter(|file| !cli.incs || Path::new(file).extension() == Some(OsStr::new("inc")))
        .filter(|file| !cli.locals || Path::new(file).extension() == Some(OsStr::new("local")))
        .filter(|file| !cli.profiles || Path::new(file).extension() == Some(OsStr::new("profile")))
        .collect::<Vec<_>>();
    user_profiles.sort_unstable();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    for user_profile in user_profiles {
        stdout.write_all(user_profile.as_bytes()).unwrap();
        stdout.write_all(b"\n").unwrap();
    }
}
