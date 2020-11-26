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

//! Module for dealing with profiles

#![allow(clippy::unreadable_literal)] // bitflags are easier to read without underscores!!
#![allow(dead_code)] // Some methods are for future use, others are USED! (=false positive)

use crate::location::Location;
use crate::{SYSTEM_PROFILE_DIR, USER_PROFILE_DIR};
use bitflags::bitflags;
use lazy_static::lazy_static;
use log::{debug, warn};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::fs::{read_dir, read_to_string};
use std::io;
use std::path::{Path, PathBuf};

lazy_static! {
    /// `lazy_static`: HashMap with the shortnames used by [`complete_name`]
    static ref SHORTNAMES: HashMap<&'static str, &'static str> = [
        ("acd", "allow-common-devel.inc"),
        ("ag", "allow-gjs.inc"),
        ("aj", "allow-java.inc"),
        ("al", "allow-lua.inc"),
        ("ap", "allow-perl.inc"),
        ("ap2", "allow-python2.inc"),
        ("ap3", "allow-python3.inc"),
        ("ar", "allow-ruby.inc"),
        ("dc", "disable-common.inc"),
        ("dd", "disable-devel.inc"),
        ("de", "disable-exec.inc"),
        ("di", "disable-interpreters.inc"),
        ("dp", "disable-programs.inc"),
        ("dpm", "disable-passwdmgr.inc"),
        ("ds", "disable-shell"),
        ("dx", "disbale-xdg.inc"),
        ("wc", "whitelist-common.inc"),
        ("wruc", "whitelist-runuser-common.inc"),
        ("wusc", "whitelist-usr-share-common.inc"),
        ("wvc", "whitelist-var-common.inc"),
    ]
    .iter()
    .copied()
    .collect();
}

bitflags! {
    /// Flags for creating a new instance of profile
    pub struct ProfileFlags: u8 {
        /// Search in the current working directory (default)
        const LOOKUP_CWD        = 0b00000001;
        /// Search under `~/.config/firejail` (default)
        const LOOKUP_USER       = 0b00000010;
        /// Search under `/etc/firejail` (default)
        const LOOKUP_SYSTEM     = 0b00000100;
        /// Read the data of the profile
        const READ              = 0b00001000;
        /// Reject profiles with a '/'
        const DENY_BY_PATH      = 0b00010000;
        /// Assume that the profile exists in the location with the highest priority
        const ASSUME_EXISTENCE  = 0b00100000;
    }
}
impl ProfileFlags {
    /// Add flag `other` to self and return the result
    ///
    /// # Examples
    ///
    /// ```
    /// ProfileFlags::default().with(ProfileFlags::READ)
    /// ```
    pub fn with(self, other: Self) -> Self {
        self | other
    }

    /// Remove flag `other` from self and return the result
    ///
    /// # Examples
    ///
    /// ```
    /// ProfileFlags::default().without(ProfileFlags::LOOKUP_CWD)
    /// ```
    pub fn without(self, other: Self) -> Self {
        self & !other
    }
}
/// Default is `LOOKUP_CWD`, `LOOKUP_USER` and `LOOKUP_SYSTEM`
impl Default for ProfileFlags {
    fn default() -> Self {
        Self::LOOKUP_CWD | Self::LOOKUP_USER | Self::LOOKUP_SYSTEM
    }
}

/// The representation of a profile
#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Profile<'a> {
    /// The raw name of the profile, passed to [`new`]
    ///
    /// [`new`]: #method.new
    raw_name: Cow<'a, str>,
    /// The completed name of the profile, maybe equal to raw_name
    full_name: Cow<'a, str>,
    /// The path to the profile
    ///
    /// This is `None` if [`new`] is called without any `LOOKUP_*` flag
    /// or no profile exists for it in the searched locations.
    ///
    /// | | `.`, user, system | `.`, user | `.`, system | user, system | `.` | user | system |
    /// | | | | | | | | |
    /// | CWD USER SYSTEM (default) | `.` | `.` | `.` | user | `.` | user | system |
    /// | CWD USER SYSTEM ASSUME | `.` | `.` | `.` | `.` | `.` | `.` | `.` |
    /// | USER | user | user | none | user | none | user | none |
    /// | USER ASSUME | user | user | user | user | user | user | user |
    ///
    /// [`new`]: #method.new
    path: Option<PathBuf>,
    /// The profile raw data
    ///
    /// This is `None` if [`new`] is called without READ flag (default),
    /// and [`read`] hasn't been called on it.
    ///
    /// [`new`]: #method.new
    /// [`read`]: #method.read
    raw_data: Option<String>,
}
impl<'a> Profile<'a> {
    /// Create a new Profile
    ///
    /// If new is called without `READ` flag, it is save to call unwrap on it.
    /// However, be aware that this may change in the future.
    ///
    /// # Errors
    ///
    /// - [`Error::ReadError`]
    ///
    /// # Panics
    ///
    /// Panics if `name` is `.` or `..`.
    ///
    /// # Examples
    ///
    /// ```
    /// // unwrap is save here, because ProfileFlags::default() does not contain READ
    /// let firefox_profile = Profile::new("firefox", ProfileFlags::default()).unwrap();
    ///
    /// let totem_profile = Profile::new(
    ///     "totem.profile",
    ///     ProfileFlags::default_with(ProfileFlags::READ),
    /// )?;
    ///
    /// let keepassxc_user_path: Option<PathBuf> = Profile::new(
    ///     "keepassxc",
    ///     ProfileFlags::LOOKUP_USER | ProfileFlags::ASSUME_EXISTENCE | ProfileFlags::DENY_BY_PATH
    /// )?.path();
    /// ```
    ///
    /// [`ErrorContext`]: struct.ErrorContext.html
    pub fn new(name: &'a str, flags: ProfileFlags) -> Result<Self, Error> {
        let raw_name = Cow::Borrowed(name);
        let full_name = complete_name(name);

        debug!("Expanded profile-name '{}' to '{}'.", raw_name, full_name);

        let path;
        if name.contains('/') {
            if flags.contains(ProfileFlags::DENY_BY_PATH) {
                path = None;
            } else if flags.contains(ProfileFlags::ASSUME_EXISTENCE) || Path::new(name).exists() {
                path = Some(PathBuf::from(name));
            } else {
                path = None;
            }
        } else {
            path = lookup_profile(&*full_name, flags);
        }

        if !flags.contains(ProfileFlags::ASSUME_EXISTENCE) {
            if let Some(ref path) = path {
                debug!("Found profile {} at '{}'", full_name, path.display());
            }
        }

        let mut new_profile = Self {
            raw_name,
            full_name,
            path,
            raw_data: None,
        };

        if flags.contains(ProfileFlags::READ) {
            let res = new_profile.read();
            if let Err(err) = res {
                return Err(Error::ReadError {
                    raw_name: new_profile.raw_name.to_string(),
                    full_name: new_profile.full_name.to_string(),
                    path: new_profile.path.unwrap_or_default(),
                    source: Box::new(err),
                });
            }
        }

        Ok(new_profile)
    }

    /// Get the raw_name of the profile (i.e. the one passed to new).
    pub fn raw_name(&self) -> &Cow<'_, str> {
        &self.raw_name
    }

    /// Get the full_name of the profile.
    pub fn full_name(&self) -> &Cow<'_, str> {
        &self.full_name
    }

    /// Get the path of the profile (if exists).
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Get the raw_data of the profile (if exists).
    pub fn raw_data(&self) -> Option<&str> {
        self.raw_data.as_deref()
    }

    /// Read the data of the profile
    ///
    /// This will re-read it if it is already readed.
    ///
    /// # Errors
    ///
    /// - [`Error::NoPath`]
    /// - [`Error::Io`]
    ///
    /// # Examples
    ///
    /// ```
    /// let mut profile = Profile::new("firefox", ProfileFlags::default())?;
    /// assert_eq!(profile.data(), &None);
    ///
    /// profile.read()?;
    /// assert_eq!(profile.data(), &Some(String::from("{Data of firefox.profile here}")));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn read(&mut self) -> Result<(), Error> {
        if let Some(ref path) = self.path {
            self.raw_data = Some(read_to_string(path)?);
            Ok(())
        } else {
            Err(Error::NoPath)
        }
    }

    /// Return true if the profile is readed (i.e. the data filed is not None), otherwise false.
    ///
    /// # Examples
    ///
    /// ```
    /// let profile = Profile::new("firefox", ProfileFlags::READ)?;
    /// assert_eq!(profile.readed(), true);
    ///
    /// let mut profile = Profile::new("firefox", ProfileFlags::default())?;
    /// assert_eq!(profile.readed(), false);
    /// profile.read()?;
    /// assert_eq!(profile.readed(), true);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn readed(&self) -> bool {
        self.raw_data.is_some()
    }
}

/// Complete a profile name
///
/// This expands shortnames and adds `.profile` if necessary.
///
/// # Panics
///
/// This functions panics if `name` contains a `/` or is equal to `.`/`..`.
pub fn complete_name(name: &str) -> Cow<'_, str> {
    if name.contains('/') {
        panic!("Profile names must not contain '/'.");
    }
    if name == "." || name == ".." {
        panic!("Profile names must not be '.' or '..'");
    }

    if let Some(long_name) = SHORTNAMES.get(name) {
        Cow::Borrowed(long_name)
    } else if name.ends_with(".inc") || name.ends_with(".local") || name.ends_with(".profile") {
        Cow::Borrowed(name)
    } else {
        Cow::Owned(name.to_string() + ".profile")
    }
}

/// Lookup for a file named `name` in every location specified in `flags`
///
/// The path to the first found profile is returned,
/// if no profile is found, `None` is returned.
/// `ASSUME_EXISTENCE` is respected.
///
/// Search order:
///   1. `LOOKUP_CWD` (`.`)
///   2. `LOOKUP_USER` (USER_PROFILE_DIR (`~/.config/firejail`))
///   3. `LOOKUP_SYSTEM` (SYSTEM_PROFILE_DIR (`/etc/firejail`))
fn lookup_profile(name: &str, flags: ProfileFlags) -> Option<PathBuf> {
    macro_rules! black_magic {
        (if $cond:expr => $location:expr) => {
            if $cond {
                if flags.contains(ProfileFlags::ASSUME_EXISTENCE) {
                    Some($location.get_profile_path(name))
                } else {
                    match read_dir($location.as_ref()) {
                        Ok(files) => files
                            .filter_map(|ent| match ent {
                                Ok(ent) => Some(ent),
                                Err(err) => {
                                    warn!("There was a error in the lookup of: {}", err);
                                    None
                                }
                            })
                            .find(|ent| ent.file_name() == name)
                            .map(|ent| ent.path()),
                        Err(err) => {
                            warn!("Failed to open {}: {}", $location, err);
                            None
                        }
                    }
                }
            } else {
                None
            }
        };
    }

    black_magic!(if flags.contains(ProfileFlags::LOOKUP_CWD) => Location::from("."))
        .or_else(
            || black_magic!(if flags.contains(ProfileFlags::LOOKUP_USER) => &*USER_PROFILE_DIR),
        )
        .or_else(
            || black_magic!(if flags.contains(ProfileFlags::LOOKUP_SYSTEM) => &*SYSTEM_PROFILE_DIR),
        )
}

/// Profile Error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Occurs when calling [`new`](Profile::new) with [`ProfileFlags::READ`] and
    /// the internal call to [`read`](Profile::read) fails.
    ///
    /// If you expect that this likely happens, you should call [`read`](Profile::read)
    /// yourself, because the creation of this variant calls `.to_string()`
    /// on `raw_name` and `full_name`
    #[error("Failed to read '{full_name}': {source}")]
    ReadError {
        /// [`Profile::raw_name`]
        raw_name: String,
        /// [`Profile::full_name`]
        full_name: String,
        /// [`Profile::path`] or [`PathBuf::defaul()`](std::path::PathBuf::default)
        path: PathBuf,
        /// The error returned by [`read`](Profile::read)
        source: Box<dyn StdError + Send + Sync>,
    },
    /// Occurs when calling [`read`](Profile::read) on a [`Profile`] without a path
    /// (i.e. [`path`](Profile::path)  is `None`).
    #[error("Called read on a Profile without a path.")]
    NoPath,
    /// Wraps an [I/O Error](std::io::Error).
    #[error("{0}")]
    Io(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_flags_with() {
        assert_eq!(
            ProfileFlags::default().with(ProfileFlags::READ),
            ProfileFlags::default() | ProfileFlags::READ,
        );
    }

    #[test]
    fn profile_flags_without() {
        assert_eq!(
            ProfileFlags::default().without(ProfileFlags::LOOKUP_CWD),
            ProfileFlags::default() & !ProfileFlags::LOOKUP_CWD,
        );
    }
}
