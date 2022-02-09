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

use crate::fatal;
use crate::profile::{Profile, ProfileFlags};
use crate::profile_stream::ProfileStream;
use crate::utils::ColoredText;
use clap::ArgMatches;
use termcolor::Color;

pub fn start(cli: &ArgMatches<'_>) {
    let [(profile1, profile1_stream), (profile2, profile2_stream)] = read_and_parse(cli);

    match cli.value_of("format") {
        Some("color") => format_color(&profile1, &profile2, &profile1_stream, &profile2_stream),
        Some("simple") => format_simple(&profile1, &profile2, &profile1_stream, &profile2_stream),
        _ => unreachable!(),
    }
}

fn read_and_parse<'a>(cli: &'a ArgMatches<'a>) -> [(Profile<'a>, ProfileStream); 2] {
    let profile1_name = cli.value_of("PROFILE_NAME1").unwrap();
    let profile2_name = cli.value_of("PROFILE_NAME2").unwrap();

    let profile1 = Profile::new(
        profile1_name,
        ProfileFlags::default().with(ProfileFlags::READ),
    )
    .unwrap_or_else(|err| fatal!("Failed to read {}: {}", profile1_name, err));
    let profile2 = Profile::new(
        profile2_name,
        ProfileFlags::default().with(ProfileFlags::READ),
    )
    .unwrap_or_else(|err| fatal!("Failed to read {}: {}", profile2_name, err));

    let profile1_stream = profile1.raw_data().parse::<ProfileStream>().unwrap();
    let profile2_stream = profile2.raw_data().parse::<ProfileStream>().unwrap();

    [(profile1, profile1_stream), (profile2, profile2_stream)]
}

fn format_color(
    profile1: &Profile<'_>,
    profile2: &Profile<'_>,
    profile1_stream: &ProfileStream,
    profile2_stream: &ProfileStream,
) {
    println!(
        "{}\n{}\n{}\n{}",
        ColoredText::new(
            Color::Cyan,
            format!("{}:", profile1.path().unwrap().to_string_lossy()),
        ),
        profile1_stream
            .iter()
            .map(|l| if profile2_stream.contains(&l.content) {
                l.content.to_string()
            } else {
                ColoredText::new(Color::Green, l.content.to_string()).into_string()
            })
            .collect::<String>(),
        ColoredText::new(
            Color::Cyan,
            format!("{}:", profile2.path().unwrap().to_string_lossy()),
        ),
        profile2_stream
            .iter()
            .map(|l| if profile1_stream.contains(&l.content) {
                l.content.to_string()
            } else {
                ColoredText::new(Color::Green, l.content.to_string()).into_string()
            })
            .collect::<String>()
    );
}

fn format_simple(
    profile1: &Profile<'_>,
    profile2: &Profile<'_>,
    profile1_stream: &ProfileStream,
    profile2_stream: &ProfileStream,
) {
    let profile1_unique = profile1_stream
        .iter()
        .filter(|l| !l.is_comment())
        .filter(|l| !profile2_stream.contains(&l.content))
        .cloned()
        .collect::<ProfileStream>();
    let profile2_unique = profile2_stream
        .iter()
        .filter(|l| !l.is_comment())
        .filter(|l| !profile1_stream.contains(&l.content))
        .cloned()
        .collect::<ProfileStream>();

    print!(
        "{}\n{}\n{}\n{}",
        ColoredText::new(
            Color::Cyan,
            format!(
                "The following commands are unique to {}:",
                profile1.full_name()
            ),
        ),
        profile1_unique,
        ColoredText::new(
            Color::Cyan,
            format!(
                "The following commands are unique to {}:",
                profile2.full_name()
            ),
        ),
        profile2_unique,
    );
}
