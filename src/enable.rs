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

use crate::{
    disable::DISABLED_DIR,
    profile::{Profile, ProfileFlags},
    utils::input,
    USER_PROFILE_DIR,
};
use clap::ArgMatches;
use log::{debug, error, info, warn};
use std::fs::rename;

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: enable");

    if cli.is_present("user") {
        enable_user();
    } else {
        let profile_name = cli.value_of("PROFILE_NAME").unwrap();
        enable_profile(
            &Profile::new(
                profile_name,
                ProfileFlags::LOOKUP_USER
                    | ProfileFlags::ASSUME_EXISTENCE
                    | ProfileFlags::DENY_BY_PATH,
            )
            .unwrap(),
        );
    }
}

fn enable_user() {
    let mut disabled_user_profile_dir = USER_PROFILE_DIR.to_owned_inner();
    disabled_user_profile_dir.set_extension("disabled");
    debug!(
        "disabled user profile dir: {}",
        disabled_user_profile_dir.to_string_lossy()
    );
    rename(&disabled_user_profile_dir, &*USER_PROFILE_DIR)
        .unwrap_or_else(|err| error!("Rename failed: {}", err));
}

fn enable_profile(profile: &Profile<'_>) {
    let disabled_profile = DISABLED_DIR.get_profile_path(profile.full_name());
    debug!("disabled profile: {}", disabled_profile.to_string_lossy());

    if !disabled_profile.exists() {
        error!("{} is not disabled.", profile.full_name());
        return;
    }

    // NOTE: unwrap can't fail because profile is created with ASSUME_EXISTENCE.
    let enabled_profile = profile.path().unwrap();
    debug!("enabled profile: {}", enabled_profile.to_string_lossy());

    if enabled_profile.exists() {
        warn!("Profile '{}' is alread enabled.", profile.full_name());
        if input("Override? [Y/n] ").unwrap() != "y" {
            info!("Skipping");
            return;
        }
    }

    rename(&disabled_profile, &enabled_profile)
        .unwrap_or_else(|err| error!("Rename failed: {}", err));
}
