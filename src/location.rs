/*
 * Copyright © 2020,2021 The fjp Authors
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

//! Module for dealing with the directories where profiles are stored

#![allow(dead_code)]

use std::convert::From;
use std::fmt;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

/// A directory where firejail-profiles are stored, such as `/etc/firejail/`
///
/// # Examples
///
/// ```
/// let system_l = Location::from("/etc/firejail/")
/// let firefox_profile_path;
/// if system.has_profile("firefox.profile") {
///     firefox_profile_path = system_l.get_profile_path("firefox.profile");
/// }
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Location {
    inner: PathBuf,
}

impl Location {
    /// Get the path to profile named `name` in this location
    ///
    /// NOTE: This does not check if `name` exists.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(
    ///     Location::from("/etc/firejail/").get_profile_path("firefox.profile"),
    ///     PathBuf::from("/etc/firejail/firefox.profile"),
    /// );
    /// ```
    pub fn get_profile_path(&self, name: &str) -> PathBuf {
        let mut p = self.inner.to_path_buf();
        p.push(name);
        p
    }

    /// Check if a file named `name` exists in this location
    pub fn has_profile(&self, name: &str) -> IoResult<bool> {
        for entry in self.inner.read_dir()? {
            if entry?.file_name() == name {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn get_ref(&self) -> &Path {
        &self.inner
    }

    /// Clone the inner PathBuf and return it
    pub fn to_owned_inner(&self) -> PathBuf {
        self.inner.to_path_buf()
    }
}

impl AsRef<Path> for Location {
    fn as_ref(&self) -> &Path {
        &self.inner
    }
}

impl<T: Into<PathBuf>> From<T> for Location {
    fn from(p: T) -> Self {
        Self { inner: p.into() }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.inner.to_string_lossy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_profile_path() {
        assert_eq!(
            Location::from("/").get_profile_path("MyProfile"),
            PathBuf::from("/MyProfile"),
        );
    }

    #[test]
    fn test_get_ref() {
        assert_eq!(Location::from("/").get_ref(), Path::new("/"));
    }

    #[test]
    fn test_to_owned_inner() {
        assert_eq!(Location::from("/").to_owned_inner(), PathBuf::from("/"));
    }
}
