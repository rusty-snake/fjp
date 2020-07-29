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

use crate::{SYSTEM_PROFILE_DIR, USER_PROFILE_DIR};
use anyhow::anyhow;
use bitflags::bitflags;
use lazy_static::lazy_static;
use log::{debug, warn};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error as StdError;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
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
    pub struct NewProfileFlags: u8 {
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
    }
}
impl NewProfileFlags {
    /// Create a new instance with the default flags and the provided additional flags
    pub fn default_with(additional_flags: Self) -> Self {
        let mut flags = Self::default();
        flags.insert(additional_flags);
        flags
    }
}
/// Default is `LOOKUP_CWD`, `LOOKUP_USER` and `LOOKUP_SYSTEM`
impl Default for NewProfileFlags {
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
    path: Option<PathBuf>,
    /// The profile raw data
    ///
    /// This is `None` if [`new`] is called without READ flag (default),
    /// and [`read`] hasn't been called on it.
    raw_data: Option<String>,
}
impl<'a> Profile<'a> {
    /// Create a new Profile
    ///
    /// If new is called without READ flag, it is save to call unwrap on it.
    /// However, be aware that this may change in the future.
    ///
    /// # Errors
    ///
    /// This function can ATM only return an error if `NewProfileFlags::READ` was give.
    /// This error can be downcasted to `std::io::Error` or [`ErrorContext`].
    ///
    /// # Panics
    ///
    /// See [`lookup_profile`](#method.lookup_profile).
    ///
    /// # Examples
    ///
    /// ```
    /// // unwrap is save here, because NewProfileFlags::default() does not contain READ
    /// let firefox_profile = Profile::new("firefox", NewProfileFlags::default()).unwrap();
    ///
    /// let totem_profile = Profile::new(
    ///     "totem.profile",
    ///     NewProfileFlags::default_with(NewProfileFlags::READ),
    /// )?;
    /// ```
    ///
    /// [`ErrorContext`]: struct.ErrorContext.html
    pub fn new(name: &'a str, flags: NewProfileFlags) -> anyhow::Result<Self> {
        let raw_name = Cow::Borrowed(name);
        let full_name = Self::complete_name(name);

        debug!("Expanded profile-name '{}' to '{}'.", raw_name, full_name);

        let path;
        if name.contains('/') {
            if flags.contains(NewProfileFlags::DENY_BY_PATH) {
                path = None;
            } else if Path::new(name).exists() {
                path = Some(PathBuf::from(name));
            } else {
                path = None;
            }
        } else {
            path = Self::lookup_profile(OsStr::new(&*full_name), flags);
        }

        if let Some(ref path) = path {
            debug!(
                "Found profile {} at '{}'",
                full_name,
                path.to_string_lossy(),
            );
        }

        let mut new_profile = Self {
            raw_name,
            full_name,
            path,
            raw_data: None,
        };

        if flags.contains(NewProfileFlags::READ) {
            let res = new_profile.read();
            if let Err(err) = res {
                return Err(anyhow::Error::new(err).context(ErrorContext::from(new_profile)));
            }
        }

        Ok(new_profile)
    }

    /// Get the raw_name of the profile.
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
    /// # Examples
    ///
    /// ```
    /// let mut profile = Profile::new("firefox", NewProfileFlags::default())?;
    /// assert_eq!(profile.data(), &None);
    ///
    /// profile.read()?;
    /// assert_eq!(profile.data(), &Some(String::from("{Data of firefox.profile here}")));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn read(&mut self) -> io::Result<()> {
        use io::{Error, ErrorKind};
        if let Some(ref path) = self.path {
            self.raw_data = Some(read_to_string(path)?);
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                anyhow!("This profile does not exists"),
            ))
        }
    }

    /// Return true if the profile is readed (i.e. the data filed is not None), otherwise false.
    ///
    /// # Examples
    ///
    /// ```
    /// let profile = Profile::new("firefox", NewProfileFlags::READ)?;
    /// assert_eq!(profile.readed(), true);
    ///
    /// let mut profile = Profile::new("firefox", NewProfileFlags::default())?;
    /// assert_eq!(profile.readed(), false);
    /// profile.read()?;
    /// assert_eq!(profile.readed(), true);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn readed(&self) -> bool {
        self.raw_data.is_some()
    }

    /// Complete a profile name
    ///
    /// This expands shortnames and adds `.profile` if necessary.
    ///
    /// # Panics
    ///
    /// This functions panics if `name` contains a `/` or is equal to `.`/`..`.
    fn complete_name(name: &str) -> Cow<'_, str> {
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
    ///
    /// Search order:
    ///   1. `LOOKUP_CWD` (`.`)
    ///   2. `LOOKUP_USER` (USER_PROFILE_DIR (`~/.config/firejail`))
    ///   3. `LOOKUP_SYSTEM` (SYSTEM_PROFILE_DIR (`/etc/firejail`))
    fn lookup_profile(name: &OsStr, flags: NewProfileFlags) -> Option<PathBuf> {
        if flags.contains(NewProfileFlags::LOOKUP_CWD) {
            Self::lookup_dir(name, ".").unwrap_or_else(|err| {
                warn!(
                    "An error occurred while search in the current working directory: {}",
                    err
                );
                None
            })
        } else {
            None
        }
        .or_else(|| {
            if flags.contains(NewProfileFlags::LOOKUP_USER) {
                Self::lookup_dir(name, &*USER_PROFILE_DIR).unwrap_or_else(|err| {
                    warn!(
                        "An error occurred while search in the user profile directory: {}",
                        err
                    );
                    None
                })
            } else {
                None
            }
        })
        .or_else(|| {
            if flags.contains(NewProfileFlags::LOOKUP_SYSTEM) {
                Self::lookup_dir(name, &*SYSTEM_PROFILE_DIR).unwrap_or_else(|err| {
                    warn!(
                        "An error occurred while search in the system profile directory: {}",
                        err
                    );
                    None
                })
            } else {
                None
            }
        })
    }

    /// Check if `path` contains `name`
    ///
    /// If so `Ok(Some(PathBuf::from(path+name)))` is returned, otherwise `Ok(None)`
    fn lookup_dir<N, P>(name: N, path: P) -> io::Result<Option<PathBuf>>
    where
        N: AsRef<OsStr>,
        P: AsRef<Path>,
    {
        for etr in read_dir(path.as_ref())? {
            let etr = etr?;
            if etr.file_name() == name.as_ref() {
                return Ok(Some(etr.path()));
            }
        }
        Ok(None)
    }
}

/// Context information of an error
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorContext {
    pub raw_name: String,
    pub full_name: String,
    pub path: String,
}
impl StdError for ErrorContext {}
/// NOTE: This is more a dummy implementation to work with anyhow then an usefull message.
impl Display for ErrorContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Name '{}' completed to '{}' and found at '{}'.",
            self.raw_name, self.full_name, self.path,
        )
    }
}
impl From<Profile<'_>> for ErrorContext {
    fn from(profile: Profile) -> Self {
        Self {
            raw_name: profile.raw_name.into_owned(),
            full_name: profile.full_name.into_owned(),
            path: profile.path.map_or_else(
                || "Not found".to_string(),
                |p| {
                    p.into_os_string()
                        .into_string()
                        .unwrap_or_else(|os| os.to_string_lossy().into_owned())
                },
            ),
        }
    }
}
