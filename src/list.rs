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

use crate::{USER_PROFILE_DIR, fatal};
use clap::ArgMatches;
use log::{debug, warn};
use std::fs::read_dir;

pub fn start(_cli: &ArgMatches<'_>) {
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
        .collect::<Vec<_>>();
    user_profiles.sort_unstable();
    print!(
        "{}",
        user_profiles.into_iter().fold(String::new(), |mut acc, val| {
            acc.push_str(val.to_str().unwrap());
            acc.push('\n');
            acc
        })
    );
}
