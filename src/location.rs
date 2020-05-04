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

use crate::profile::Profile;
use crate::utils::AddTo;
use std::borrow::Cow;
use std::convert::From;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Location<'a> {
    inner: Cow<'a, Path>,
}

impl Location<'_> {
    pub fn new_join<P1, P2>(path1: P1, path2: P2) -> Self
    where
        P1: Into<PathBuf>,
        P2: AsRef<Path>,
    {
        Self {
            inner: Cow::from(path2.as_ref().add_to(path1.into())),
        }
    }

    pub fn get_profile<'a>(&self, name: &'a str) -> Profile<'a> {
        Profile {
            name,
            path: {
                let mut p = self.inner.to_path_buf();
                p.push(name);
                p
            },
            data: None,
        }
    }

    pub fn get_ref(&self) -> &Path {
        &self.inner
    }

    pub fn to_owned_inner(&self) -> PathBuf {
        self.inner.to_path_buf()
    }
}

impl AsRef<Path> for Location<'_> {
    fn as_ref(&self) -> &Path {
        &self.inner
    }
}

impl<'a> From<&'a Path> for Location<'a> {
    fn from(p: &'a Path) -> Self {
        Self {
            inner: Cow::Borrowed(p),
        }
    }
}

impl From<PathBuf> for Location<'_> {
    fn from(p: PathBuf) -> Self {
        Self {
            inner: Cow::Owned(p),
        }
    }
}

impl<'a> From<&'a str> for Location<'a> {
    fn from(s: &'a str) -> Self {
        Self::from(Path::new(s))
    }
}

impl From<String> for Location<'_> {
    fn from(s: String) -> Self {
        Self::from(PathBuf::from(s))
    }
}

impl<'a> From<&'a OsStr> for Location<'a> {
    fn from(s: &'a OsStr) -> Self {
        Self::from(Path::new(s))
    }
}

impl From<OsString> for Location<'_> {
    fn from(s: OsString) -> Self {
        Self::from(PathBuf::from(s))
    }
}

impl fmt::Display for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.inner.to_string_lossy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_join() {
        assert_eq!(
            Location::new_join("/java", "script"),
            Location {
                inner: Cow::from(Path::new("/java/script"))
            },
        );
    }

    #[test]
    fn test_get_profile() {
        assert_eq!(
            Location::from("/").get_profile("MyProfile"),
            Profile {
                name: "MyProfile",
                path: PathBuf::from("/MyProfile"),
                data: None
            },
        );
    }

    #[test]
    fn test_get_ref() {
        assert_eq!(Location::from("/").get_ref(), Path::new("/"),);
    }

    #[test]
    fn test_to_owned_inner() {
        assert_eq!(Location::from("/").to_owned_inner(), PathBuf::from("/"),);
    }
}
