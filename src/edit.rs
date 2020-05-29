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

#![allow(clippy::unreadable_literal)] // bitflags are easier to read without underscores!!

use crate::{
    fatal, profile_path,
    utils::{get_name1, PushExtension},
};
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
        const CREATE = 0b00000010;
        const TMP    = 0b00000100;
    }
}

pub fn start(cli: &ArgMatches<'_>) {
    debug!("subcommand: edit");

    #[rustfmt::skip]
    let flags =
        if cli.is_present("no-copy") { Flags::NULL } else { Flags::COPY }
        | if cli.is_present("no-create") { Flags::NULL } else { Flags::CREATE }
        | if cli.is_present("tmp") { Flags::TMP } else { Flags::NULL };

    // NOTE: unwrap can't faile here, because PROFILE_NAME is required
    let profile_name = get_name1(cli.value_of("PROFILE_NAME").unwrap());
    debug!("profile name: {}", profile_name);

    let system_profile = profile_path!(SYSTEM / &profile_name);
    debug!("system profile: {}", system_profile.to_string_lossy());

    let user_profile = profile_path!(USER / &profile_name);
    debug!("user profile: {}", user_profile.to_string_lossy());

    if flags.contains(Flags::TMP) {
        prepare_tmp_edit(&user_profile, &system_profile, flags);
    } else {
        prepare_edit(&user_profile, &system_profile, flags);
    }
}

fn prepare_tmp_edit(user_profile: &Path, system_profile: &Path, flags: Flags) {
    let backup_profile = user_profile.to_path_buf().push_extension(".bak");

    if user_profile.exists() {
        rename(user_profile, &backup_profile)
            .unwrap_or_else(|err| fatal!("backup creation failed: {}", err));
        prepare_edit(user_profile, system_profile, flags);
        rename(&backup_profile, user_profile)
            .unwrap_or_else(|err| fatal!("failed to restore the profile: {}", err));
    } else {
        prepare_edit(user_profile, system_profile, flags);
        remove_file(user_profile)
            .unwrap_or_else(|err| fatal!("failed to remove the temporary profile: {}", err));
    }
}

fn prepare_edit(user_profile: &Path, system_profile: &Path, flags: Flags) {
    if flags.contains(Flags::COPY) && !user_profile.exists() && system_profile.exists() {
        info!("copying the profile");
        copy_file(&system_profile, &user_profile).unwrap_or_else(|err| {
            fatal!(
                "Failed to copy '{}' to '{}': {}",
                system_profile.to_string_lossy(),
                user_profile.to_string_lossy(),
                err
            )
        });
    }

    if flags.intersects(Flags::COPY | Flags::CREATE) || user_profile.exists() {
        open_user_profile(user_profile);
    }
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
