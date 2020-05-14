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

use clap::{crate_description, load_yaml, App};
use env_logger::{Builder, Env};
use lazy_static::lazy_static;

mod location;
mod profile;
mod utils;

use location::Location;
use utils::home_dir;

mod cat;
mod disable;
mod edit;
mod enable;
mod has;
mod list;
mod rm;

use cat::start as start_cat;
use disable::start as start_disable;
use edit::start as start_edit;
use enable::start as start_enable;
use has::start as start_has;
use list::start as start_list;
use rm::start as start_rm;

lazy_static! {
    static ref SYSTEM_PROFILE_DIR: Location<'static> = Location::from("/etc/firejail/");
    static ref USER_PROFILE_DIR: Location<'static> = {
        Location::new_join(
            home_dir().expect("Can not get user home dir. Is $HOME set?"),
            ".config/firejail/",
        )
    };
}

macros::fjp_version!();

fn main() {
    #[cfg(feature = "full")]
    color_backtrace::install();

    Builder::from_env(Env::new().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
        .about(crate_description!())
        .version(FJP_VERSION)
        .get_matches();
    match matches.subcommand() {
        ("cat", Some(sub_matches)) => start_cat(sub_matches),
        ("disable", Some(sub_matches)) => start_disable(sub_matches),
        ("edit", Some(sub_matches)) => start_edit(sub_matches),
        ("enable", Some(sub_matches)) => start_enable(sub_matches),
        ("has", Some(sub_matches)) => start_has(sub_matches),
        ("list", Some(sub_matches)) => start_list(sub_matches),
        ("rm", Some(sub_matches)) => start_rm(sub_matches),
        _ => unreachable!(),
    }
}
