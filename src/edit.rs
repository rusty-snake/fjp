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

#![allow(clippy::unreadable_literal)] // bitflags are easier to read without underscores!!

use crate::fatal;
use crate::profile::{Profile, ProfileFlags};
use crate::utils::input;
use bitflags::bitflags;
use log::{debug, warn};
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

pub fn start(cli: &crate::cli::CliEdit) {
    debug!("subcommand: edit");

    let mut flags = Flags::empty();
    if cli.tmp {
        flags.insert(Flags::TMP | Flags::COPY);
    }

    debug!("profile name: {}", cli.profile_name);

    let user_profile = Profile::new(
        &cli.profile_name,
        ProfileFlags::LOOKUP_USER | ProfileFlags::DENY_BY_PATH | ProfileFlags::ASSUME_EXISTENCE,
    )
    .unwrap()
    .into_pathbuf();

    let system_profile = Profile::new(
        &cli.profile_name,
        ProfileFlags::LOOKUP_SYSTEM | ProfileFlags::DENY_BY_PATH | ProfileFlags::ASSUME_EXISTENCE,
    )
    .unwrap()
    .into_pathbuf();

    if flags.contains(Flags::TMP) {
        prepare_tmp_edit(&user_profile, &system_profile, flags);
    } else {
        prepare_edit(&user_profile, &system_profile, flags);
    }
}

fn prepare_tmp_edit(user_profile: &Path, system_profile: &Path, flags: Flags) {
    if user_profile.exists() {
        let backup_profile = user_profile.with_extension("bak");

        debug!(
            "Copy '{}' to '{}'.",
            user_profile.display(),
            backup_profile.display()
        );
        copy_file(user_profile, &backup_profile).unwrap_or_else(|err| {
            fatal!(
                "Failed to create backup of {}: {}",
                user_profile.file_name().unwrap().to_string_lossy(),
                err
            )
        });

        prepare_edit(user_profile, system_profile, flags);

        debug!(
            "Move '{}' back to '{}'.",
            backup_profile.display(),
            user_profile.display()
        );
        rename(&backup_profile, user_profile).unwrap_or_else(|err| {
            fatal!(
                "Failed to restore {}: {}",
                user_profile.file_name().unwrap().to_string_lossy(),
                err
            )
        });
    } else {
        prepare_edit(user_profile, system_profile, flags);

        debug!("Remove '{}'.", user_profile.display());
        remove_file(user_profile)
            .unwrap_or_else(|err| fatal!("Failed to remove '{}': {}", user_profile.display(), err));
    }
}

fn prepare_edit(user_profile: &Path, system_profile: &Path, flags: Flags) {
    let copy_system_profile_to_user_profile = || {
        debug!(
            "Copy '{}' to '{}'.",
            system_profile.display(),
            user_profile.display(),
        );
        copy_file(&system_profile, &user_profile).unwrap_or_else(|err| {
            fatal!(
                "Failed to copy '{}' to '{}': {}",
                system_profile.display(),
                user_profile.display(),
                err
            )
        });
    };

    if system_profile.exists() && (flags.contains(Flags::TMP) || !user_profile.exists()) {
        if flags.contains(Flags::COPY) {
            copy_system_profile_to_user_profile();
        } else {
            match input("Should the profile be copied? [(y)es/(n)o/(a)bort] ")
                .unwrap()
                .to_lowercase()
                .as_str()
            {
                "y" => copy_system_profile_to_user_profile(),
                "n" => (),
                "a" => return,
                _ => println!("Invalid answer, continue without copying."),
            }
        }
    }

    open_user_profile(user_profile);
}

fn open_user_profile(profile: &Path) {
    let editor = var_os("EDITOR").unwrap_or_else(|| {
        warn!("$EDITOR not set or empty, using \"vim\" as fallback.");
        OsString::from("vim")
    });

    debug!(
        "Open '{}' with {}.",
        profile.display(),
        editor.to_string_lossy()
    );
    let exit_code = Command::new(&editor)
        .arg(profile)
        .status()
        .unwrap_or_else(|err| fatal!("Failed to start {}: {}", editor.to_string_lossy(), err));
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
