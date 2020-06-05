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

//! Module for various helper functions, macros, types, traits, ...

#![allow(dead_code)] // This module acts more like a library, so not yet used is ok.

use log::debug;
use std::env;
use std::fmt;
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

/// Get the users home directory
///
/// _ATM this only checks $HOME. TODO: `getpw*` fallback_
///
/// It returns `None` if $HOME is not set or empty,
/// otherwise it returns `Some(PathBuf::from($HOME))`.
pub fn home_dir() -> Option<path::PathBuf> {
    use env::var_os;
    use path::PathBuf;

    var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .map(PathBuf::from)
}

//
// ColoredText
//

/// A colored string
///
/// `termcolor` does not support simple coloring of string like `ansi_term` with
/// `Red.paint("Hello")` or `colored` with `"Hello".green()` does.
/// Instead you must manually write `set_color`, `write`, `reset`.
/// This type tries to fixes this.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColoredText {
    inner: String,
}
impl ColoredText {
    /// Create a new `ColoredText` instances
    pub fn new(color: termcolor::Color, text: impl AsRef<str>) -> Self {
        use termcolor::{Buffer, ColorSpec, WriteColor};

        let mut buffer = Buffer::ansi();
        buffer
            .set_color(ColorSpec::new().set_fg(Some(color)))
            .unwrap();
        buffer.write_all(text.as_ref().as_bytes()).unwrap();
        buffer.reset().unwrap();

        Self {
            inner: String::from_utf8_lossy(buffer.as_slice()).into_owned(),
        }
    }

    /// Get a references to the underlying String
    pub fn get_ref(&self) -> &String {
        &self.inner
    }

    /// Get a mutable references to the underlying String
    ///
    /// **Warning:** Be care full when editing this String, at the begin and end are
    /// ANSI-escape sequences. If you edit them, you might get ugly output.
    ///
    pub fn get_mut(&mut self) -> &mut String {
        &mut self.inner
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.inner.into_bytes()
    }

    pub fn into_string(self) -> String {
        self.inner
    }
}
impl fmt::Display for ColoredText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}
impl AsRef<[u8]> for ColoredText {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_bytes()
    }
}
impl AsRef<str> for ColoredText {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
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
