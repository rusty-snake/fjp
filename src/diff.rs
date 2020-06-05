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

use crate::fatal;
use crate::profile::{NewProfileFlags, Profile};
use crate::profile_stream::ProfileStream;
use crate::utils::ColoredText;
use clap::ArgMatches;
use termcolor::Color;

pub fn start(cli: &ArgMatches<'_>) {
    let profile1_name = cli.value_of("PROFILE_NAME1").unwrap();
    let profile2_name = cli.value_of("PROFILE_NAME2").unwrap();

    let profile1 = Profile::new(
        profile1_name,
        NewProfileFlags::default_with(NewProfileFlags::READ),
    )
    .unwrap_or_else(|err| fatal!("Failed to read {}: {}", profile1_name, err));
    let profile2 = Profile::new(
        profile2_name,
        NewProfileFlags::default_with(NewProfileFlags::READ),
    )
    .unwrap_or_else(|err| fatal!("Failed to read {}: {}", profile2_name, err));

    let profile1_stream = dbg!(profile1
        .raw_data()
        .unwrap()
        .parse::<ProfileStream>()
        .unwrap());
    let profile2_stream = dbg!(profile2
        .raw_data()
        .unwrap()
        .parse::<ProfileStream>()
        .unwrap());

    let profile1_unique = profile1_stream
        .iter()
        .filter(|l| !l.is_comment())
        .filter(|l| !profile2_stream.contains(l))
        .cloned()
        .collect::<ProfileStream>();
    let profile2_unique = profile2_stream
        .iter()
        .filter(|l| !l.is_comment())
        .filter(|l| !profile1_stream.contains(l))
        .cloned()
        .collect::<ProfileStream>();

    print!(
        "{}\n{}\n{}\n{}\n",
        ColoredText::new(
            Color::Cyan,
            format!(
                "The following options are unique to {}:",
                profile1.full_name()
            ),
        ),
        profile1_unique,
        ColoredText::new(
            Color::Cyan,
            format!(
                "The following options are unique to {}:",
                profile2.full_name()
            ),
        ),
        profile2_unique,
    );
}
