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

use crate::profile::{Profile, ProfileFlags};
use clap::ArgMatches;
use log::{debug, error, trace};
use std::fs::remove_file;

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: rm");

    let profiles = cli.values_of("PROFILE_NAMES").unwrap();
    for profile in profiles {
        let profile = Profile::new(
            profile,
            ProfileFlags::LOOKUP_USER | ProfileFlags::DENY_BY_PATH | ProfileFlags::ASSUME_EXISTENCE,
        )
        .unwrap();
        trace!("Deleting '{}'.", profile.full_name());
        remove_file(profile.path().unwrap())
            .unwrap_or_else(|err| error!("Failed to delete '{}': {}", profile.full_name(), err));
    }
}
