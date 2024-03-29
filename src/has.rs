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

use crate::profile::{Profile, ProfileFlags};
use crate::utils::ColoredText;
use log::debug;
use std::process::exit;
use termcolor::Color;

pub fn start(cli: &crate::cli::CliHas) {
    debug!("subcommand: has");

    let profile = Profile::new(&cli.profile_name, ProfileFlags::default()).unwrap();
    if let Some(path) = profile.path() {
        println!(
            "Profile found for {} at {}",
            profile.raw_name(),
            ColoredText::new(Color::Green, path.to_string_lossy())
        );
        exit(0);
    } else {
        println!("Could not find a Profile for {}.", &cli.profile_name);
        exit(100);
    }
}
