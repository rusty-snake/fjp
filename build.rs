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

use clap::{crate_description, crate_version, load_yaml, App, Shell};
use std::env::var_os;

const BIN_NAME: &str = "fjp";

fn main() {
    let out_dir = match var_os("OUT_DIR") {
        Some(out_dir) => out_dir,
        None => {
            println!("cargo:warning=Failed to generate shell completions. err:out_dir");
            return;
        }
    };

    let yaml = load_yaml!("src/cli.yaml");
    let mut app = App::from_yaml(yaml)
        .about(crate_description!())
        .version(crate_version!());

    app.gen_completions(BIN_NAME, Shell::Bash, &out_dir);
    app.gen_completions(BIN_NAME, Shell::Fish, &out_dir);
    app.gen_completions(BIN_NAME, Shell::Zsh, &out_dir);
}
