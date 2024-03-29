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
    location::Location,
    profile::{Profile, ProfileFlags},
    utils::input,
    USER_PROFILE_DIR,
};
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use std::fs::{create_dir, rename};
use std::io::Result as IoResult;

lazy_static! {
    pub static ref DISABLED_DIR: Location = {
        let mut path = USER_PROFILE_DIR.to_owned_inner();
        path.push("disabled");
        Location::from(path)
    };
}

pub fn start(cli: &crate::cli::CliDisable) {
    debug!("subcommand: disable");

    if cli.user {
        disable_user();
    } else if cli.list {
        list().unwrap_or_else(|e| error!("An error occured while listing: {}", e));
    } else {
        match create_dir(&*DISABLED_DIR) {
            Ok(()) => (),
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => (),
            Err(err) => fatal!("Failed to create the disabled dir: {}", err),
        }
        let profile = Profile::new(
            cli.profile_name.as_deref().unwrap(),
            ProfileFlags::LOOKUP_USER | ProfileFlags::DENY_BY_PATH,
        )
        .unwrap();
        disable_profile(&profile);
    }
}

fn disable_user() {
    let mut disabled_user_profile_dir = USER_PROFILE_DIR.to_owned_inner();
    disabled_user_profile_dir.set_extension("disabled");
    debug!(
        "disabled user profile dir: {}",
        disabled_user_profile_dir.to_string_lossy()
    );
    rename(&*USER_PROFILE_DIR, &disabled_user_profile_dir)
        .unwrap_or_else(|e| error!("Rename failed: {}", e));
}

fn list() -> IoResult<()> {
    for entry in DISABLED_DIR.get_ref().read_dir()? {
        println!("{}", entry?.file_name().to_string_lossy());
    }

    Ok(())
}

fn disable_profile(profile: &Profile<'_>) {
    let enabled_profile = if let Some(path) = profile.path() {
        path
    } else {
        error!(
            "Could not find '{}' in ~/.config/firejail",
            profile.full_name()
        );
        return;
    };

    let disabled_profile = DISABLED_DIR.get_profile_path(profile.full_name());

    if disabled_profile.exists() {
        warn!("Profile '{}' is alread disabled.", profile.full_name());
        if input("Override? [Y/n] ").unwrap() != "y" {
            info!("Skipping");
            return;
        }
    }

    debug!(
        "Move '{}' to '{}'",
        enabled_profile.display(),
        disabled_profile.display()
    );
    rename(enabled_profile, &disabled_profile).unwrap_or_else(|e| error!("Rename failed: {}", e));
}
