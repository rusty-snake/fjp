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

use clap::IntoApp;
use clap_complete::{generate, generate_to, Shell};
use std::env::var_os;
use std::fs::{create_dir_all, File};
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

include!("src/cli.rs");

const BIN_NAME: &str = "fjp";
const ZCOMP_HEADER: &str = r#"
#compdef fjp

autoload -U is-at-least

_profiles() {
    echo $1/*.{inc,local,profile} | sed -E "s;$1\/;;g"
}

_disabled_profiles() {
    local disabled_profiles=$(_profiles $HOME/.config/firejail/disabled)
    if [[ ${#disabled_profiles[@]} -ne 0 ]]; then
        _values 'disabled-profiles' $disabled_profiles
    fi
}

_system_profiles() {
    _values 'system-profiles' $(_profiles /etc/firejail)
}

_user_profiles() {
    _values 'user-profiles' $(_profiles $HOME/.config/firejail)
}

_all_profiles() {
    _system_profiles
    _user_profiles
}

"#;

fn main() {
    let out_dir = match var_os("FJP_SHELLCOMP_DIR").or_else(|| var_os("OUT_DIR")) {
        Some(out_dir) => out_dir,
        None => {
            println!("cargo:warning=Failed to generate shell completions. err:out_dir");
            return;
        }
    };

    create_dir_all(&out_dir).unwrap();

    let mut app = Cli::into_app();

    generate_to(Shell::Bash, &mut app, BIN_NAME, &out_dir).expect("generate_to bash");
    generate_to(Shell::Fish, &mut app, BIN_NAME, &out_dir).expect("generate_to fish");

    let mut buf = Vec::new();
    generate(Shell::Zsh, &mut app, BIN_NAME, &mut buf);
    let rzcomp = match String::from_utf8(buf) {
        Ok(comp) => comp,
        Err(err) => {
            println!("cargo:warning=Failed to generate zsh completions: {}", err);
            return;
        }
    };
    let mut zcomp = BufWriter::new(File::create(Path::new(&out_dir).join("fjp.zsh")).unwrap());
    write!(zcomp, "{}", &ZCOMP_HEADER[1..ZCOMP_HEADER.len()]).unwrap();
    let mut sub_c_arm = None;
    for line in rzcomp.lines().skip(3) {
        match line.trim() {
            "(cat)" => {
                sub_c_arm = Some("cat");
                writeln!(zcomp, "{}", line)
            }
            "(diff)" => {
                sub_c_arm = Some("diff");
                writeln!(zcomp, "{}", line)
            }
            "(disable)" => {
                sub_c_arm = Some("disable");
                writeln!(zcomp, "{}", line)
            }
            "(edit)" => {
                sub_c_arm = Some("edit");
                writeln!(zcomp, "{}", line)
            }
            "(enable)" => {
                sub_c_arm = Some("enable");
                writeln!(zcomp, "{}", line)
            }
            "(generate-standalone)" => {
                sub_c_arm = Some("generate-standalone");
                writeln!(zcomp, "{}", line)
            }
            "(has)" => {
                sub_c_arm = Some("has");
                writeln!(zcomp, "{}", line)
            }
            "(rm)" => {
                sub_c_arm = Some("rm");
                writeln!(zcomp, "{}", line)
            }
            ";;" => {
                sub_c_arm = None;
                writeln!(zcomp, "{}", line)
            }
            _ => match sub_c_arm {
                None => writeln!(zcomp, "{}", line),
                Some("cat") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_all_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                Some("diff") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_all_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                Some("disable") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_user_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                Some("edit") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_all_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                Some("enable") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_disabled_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                Some("generate-standalone") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_all_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                Some("has") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_all_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                Some("rm") => {
                    if line.contains("_files") {
                        writeln!(zcomp, "{}", line.replace("_files", "_user_profiles"))
                    } else {
                        writeln!(zcomp, "{}", line)
                    }
                }
                _ => unreachable!(),
            },
        }
        .unwrap();
    }
}
