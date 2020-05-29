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

use log::debug;
use log::{info, trace};
use std::env;
use std::error;
use std::ffi;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path;

/// Call `error!` from the log crate and exit with exit-code 1 afterwards.
#[macro_export]
macro_rules! fatal {
    (target: $target:expr, $($arg:tt)+) => {{
        ::log::error!(target: $target, $($arg)+);
        std::process::exit(1);
    }};
    ($($arg:tt)+) => {{
        ::log::error!($($arg)+);
        std::process::exit(1);
    }};
}

/// Python like `input()`.
pub fn input(prompt: &str) -> io::Result<String> {
    let mut stdout = io::stdout();
    stdout.write_all(prompt.as_bytes())?;
    stdout.flush()?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}

#[macro_export]
macro_rules! profile_path {
    (USER/$name:expr) => {{
        let mut profile_p = crate::USER_PROFILE_DIR.to_owned_inner();
        profile_p.push($name);
        profile_p
    }};
    (SYSTEM/$name:expr) => {{
        let mut profile_p = crate::SYSTEM_PROFILE_DIR.to_owned_inner();
        profile_p.push($name);
        profile_p
    }};
}

/// Return the path to profile `name` or None if it is not found.
///
/// It will first look under `USER_PROFILE_DIR` and if it is not found there,
/// then under `SYSTEM_PROFILE_DIR`.
pub fn find_profile(name: &str) -> Option<path::PathBuf> {
    // Try user profile.
    let profile = profile_path!(USER / name);
    if profile.exists() {
        debug!(
            target: "fjp::utils::find_profile",
            "Profile '{}' found at {}.",
            name,
            profile.to_string_lossy(),
        );
        return Some(profile);
    }

    // Try system profile.
    let profile = profile_path!(SYSTEM / name);
    if profile.exists() {
        debug!(
            target: "fjp::utils::find_profile",
            "Profile '{}' found at {}.",
            name,
            profile.to_string_lossy(),
        );
        return Some(profile);
    }

    debug!(target: "fjp::utils::find_profile", "Could not find profile {}.", name);
    None
}

pub fn get_name1(raw: &str) -> String {
    if raw.contains("..") {
        panic!("'..' is not allowed inside a profile name.");
    }
    if raw.ends_with(".inc") || raw.ends_with(".local") || raw.ends_with(".profile") {
        raw.to_string()
    } else {
        raw.to_string() + ".profile"
    }
}

pub fn home_dir() -> Option<path::PathBuf> {
    use env::var_os;
    use path::PathBuf;

    var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .map(PathBuf::from)
}

//
// which
//

/// function similar to which (1)
///
/// The two main modes are:
/// 1. Passing a absolute path (e.g. `/usr/bin/vim`)
/// 2. Passing a program name (e.g. `vim`)
///
/// The first will check if `/usr/bin/vim` exists and return `Ok(PathBuf::from("/usr/bin/vim"))`.
/// The second one will search `vim` in `$PATH`
/// and return the full path to the first found program.
///
/// # Errors
///
/// TODO
///
/// # Examples
///
/// ```
/// use crate::utils::which;
///
/// let browser = which("firefox")?;
/// let email = which("/usr/bin/thunderbird")?;
///
/// use std::env::args_os;
/// match which(args_os().nth(1)) {
///     Ok(path) => println!("found: {}", path.display()),
///     Err(err) => eprintln!("error: {}", err),
/// }
/// ```
///
pub fn which<T: AsRef<ffi::OsStr>>(bin: T) -> anyhow::Result<path::PathBuf> {
    use anyhow::{anyhow, bail};
    use env::{split_paths, var_os};
    use fs::read_dir;
    use path::Path;

    let bin = bin.as_ref();

    let bin_p = Path::new(bin);
    if bin_p.is_absolute() {
        if bin_p.exists() {
            Ok(bin_p.to_path_buf())
        } else {
            Err(anyhow!("{} does not exists", bin_p.to_string_lossy()))
        }
    } else {
        let paths = if let Some(paths) = var_os("PATH").filter(|s| !s.is_empty()) {
            paths
        } else {
            bail!("$PATH is not set or empty");
        };

        let mut full_path = None;
        'search: for path in split_paths(&paths) {
            debug!("Processing path '{}'", path.to_string_lossy());
            if let Ok(entrys) = read_dir(&path) {
                for entry in entrys {
                    let entry = entry?;
                    trace!("Processing file '{}'", entry.file_name().to_string_lossy());
                    if entry.file_name() == bin {
                        full_path = Some(entry.path());
                        break 'search;
                    }
                }
            } else {
                info!("Failed to read_dir '{}'", path.to_string_lossy());
            }
        }

        if let Some(path) = full_path {
            Ok(path)
        } else {
            Err(anyhow!(
                "Could not find '{}' in $PATH",
                bin.to_string_lossy()
            ))
        }
    }
}

//
// ColoredText
//

#[derive(Clone, Debug, PartialEq)]
pub struct ColoredText {
    inner: String,
}
impl ColoredText {
    pub fn new(color: termcolor::Color, text: &str) -> Self {
        use termcolor::{Buffer, ColorSpec, WriteColor};

        let mut buffer = Buffer::ansi();
        buffer
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        buffer.write_all(text.as_bytes()).unwrap();
        buffer.reset().unwrap();

        Self {
            inner: String::from_utf8_lossy(buffer.as_slice()).into_owned(),
        }
    }
}
impl fmt::Display for ColoredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

pub trait AddTo<T> {
    fn add_to(&self, other: T) -> T;
}
impl AddTo<path::PathBuf> for path::Path {
    fn add_to(&self, mut other: path::PathBuf) -> path::PathBuf {
        other.push(self);
        other
    }
}

pub trait PushExtension {
    fn push_extension(self, new_ext: impl AsRef<ffi::OsStr>) -> Self;
}
impl PushExtension for path::PathBuf {
    fn push_extension(mut self, new_ext: impl AsRef<ffi::OsStr>) -> Self {
        let mut ext = self
            .extension()
            .unwrap_or_else(|| ffi::OsStr::new(""))
            .to_os_string();
        ext.push(new_ext);
        self.set_extension(ext);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_dir() {
        env::set_var("HOME", "/home/github");
        assert_eq!(home_dir(), Some(path::PathBuf::from("/home/github")));
    }

    #[test]
    fn test_home_dir_empty() {
        env::set_var("HOME", "");
        assert_eq!(home_dir(), None);
    }

    #[test]
    fn test_home_dir_unset() {
        env::remove_var("HOME");
        assert_eq!(home_dir(), None);
    }

    #[test]
    fn test_get_name1() {
        assert_eq!(get_name1("firefox"), "firefox.profile");
        assert_eq!(get_name1("firefox.profile"), "firefox.profile");
        assert_eq!(get_name1("firefox.local"), "firefox.local");
        assert_eq!(get_name1("firefox.inc"), "firefox.inc");
        // Should not happen in real because PROFILE_NAME is required on the subcommands
        assert_eq!(get_name1(""), ".profile");
    }

    #[test]
    #[should_panic(expected = "'..' is not allowed inside a profile name.")]
    fn test_get_name1_dotdot_in_name() {
        get_name1("./../forbidden");
    }
}
