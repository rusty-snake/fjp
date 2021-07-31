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

#![allow(clippy::unreadable_literal)] // bitflags are easier to read without underscores!!

use crate::fatal;
use crate::profile::{Profile, ProfileFlags};
use crate::utils::input;
use bitflags::bitflags;
use clap::ArgMatches;
use log::{debug, info, warn};
use std::env::var_os;
use std::ffi::OsString;
use std::fs::{copy as copy_file, remove_file, rename};
use std::path::Path;
use std::process::Command;

bitflags! {
    struct Flags: u8 {
        const NULL   = 0b00000000;
        const COPY   = 0b00000001;
        const TMP    = 0b00000100;
    }
}

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: edit");

    let mut flags = Flags::empty();
    if cli.is_present("tmp") {
        flags.insert(Flags::TMP | Flags::COPY);
    }

    // NOTE: unwrap can't fail here, because PROFILE_NAME is required
    let profile_name = cli.value_of("PROFILE_NAME").unwrap();

    debug!("profile name: {}", profile_name);

    let user_profile = Profile::new(
        profile_name,
        ProfileFlags::LOOKUP_USER | ProfileFlags::DENY_BY_PATH | ProfileFlags::ASSUME_EXISTENCE,
    )
    .unwrap();

    debug!("user profile: {}", user_profile.full_name());

    let system_profile = Profile::new(
        profile_name,
        ProfileFlags::LOOKUP_SYSTEM | ProfileFlags::DENY_BY_PATH | ProfileFlags::ASSUME_EXISTENCE,
    )
    .unwrap();

    debug!("system profile: {}", system_profile.full_name());

    if let (Some(user_path), Some(system_path)) = (user_profile.path(), system_profile.path()) {
        if flags.contains(Flags::TMP) {
            prepare_tmp_edit(user_path, system_path, flags);
        } else {
            prepare_edit(user_path, system_path, flags);
        }
    }
}

fn prepare_tmp_edit(user_profile: &Path, system_profile: &Path, flags: Flags) {
    let backup_profile = user_profile.with_extension("bak");

    if user_profile.exists() {
        info!("Profile already exists, creating a backup.");
        copy_file(user_profile, &backup_profile)
            .unwrap_or_else(|err| fatal!("backup creation failed: {}", err));

        prepare_edit(user_profile, system_profile, flags);

        info!("Restoring the backup.");
        rename(&backup_profile, user_profile)
            .unwrap_or_else(|err| fatal!("failed to restore the profile: {}", err));
    } else {
        prepare_edit(user_profile, system_profile, flags);

        info!("Removing the temporary profile.");
        remove_file(user_profile)
            .unwrap_or_else(|err| fatal!("failed to remove the temporary profile: {}", err));
    }
}

fn prepare_edit(user_profile: &Path, system_profile: &Path, flags: Flags) {
    let copy_system_profile2user_profile = || {
        info!(
            "Copy '{}' to '{}'.",
            user_profile.to_string_lossy(),
            system_profile.to_string_lossy()
        );
        copy_file(&system_profile, &user_profile).unwrap_or_else(|err| {
            fatal!(
                "Failed to copy '{}' to '{}': {}",
                system_profile.to_string_lossy(),
                user_profile.to_string_lossy(),
                err
            )
        });
    };

    if system_profile.exists() && (flags.contains(Flags::TMP) || !user_profile.exists()) {
        if flags.contains(Flags::COPY) {
            copy_system_profile2user_profile();
        } else {
            match input("Should the profile be copied? [(y)es/(n)o/(a)bort] ")
                .unwrap()
                .to_lowercase()
                .as_str()
            {
                "y" => copy_system_profile2user_profile(),
                "n" => (),
                "a" => return,
                _ => println!("Invalid answer, continue without copying."),
            }
        }
    }

    open_user_profile(user_profile);
}

fn open_user_profile(profile: &Path) {
    let editor = var_os("EDITOR").unwrap_or_else(|| OsString::from("vim"));

    debug!("open editor with: {}", profile.to_string_lossy());
    let mut child = Command::new(&editor)
        .arg(profile)
        .spawn()
        .unwrap_or_else(|e| fatal!("Could not start {}: {}", editor.to_string_lossy(), e));
    let exit_code = child.wait().unwrap();
    if !exit_code.success() {
        warn!(
            "{} exited with exit code {}",
            editor.to_string_lossy(),
            exit_code
                .code()
                .map_or("unknow".to_string(), |c| c.to_string())
        );
    }
}
