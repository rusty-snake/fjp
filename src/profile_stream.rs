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

//! Abstract representations of option in a profile

#![allow(clippy::cognitive_complexity)]
#![allow(clippy::unreadable_literal)] // bitflags are easier to read without underscores!!

use anyhow::bail;
use bitflags::bitflags;
use std::borrow::{Borrow, BorrowMut, Cow};
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;
use std::slice;
use std::str::FromStr;
use std::sync::Arc;
use std::vec;

pub type CowArcProfileEntry<'a> = Cow<'a, Arc<ProfileEntry>>;

/// An abstract stream of lines in a firejail profile
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileStream<'a> {
    inner: Vec<CowArcProfileEntry<'a>>,
}
impl ProfileStream<'_> {
    /// Return `true` if this `ProfileStream` contains `entry`, otherwise `false`
    #[inline]
    pub fn contains(&self, entry: &ProfileEntry) -> bool {
        self.inner.iter().any(|p| p == entry)
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, CowArcProfileEntry> {
        self.inner.iter()
    }

    /// Extracts a slice containing the entire underlying vector
    #[inline]
    pub fn as_slice(&self) -> &[CowArcProfileEntry<'_>] {
        &self.inner[..]
    }
}
impl<'a> ProfileStream<'a> {
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'a, CowArcProfileEntry> {
        self.inner.iter_mut()
    }

    /// Extracts a mutable slice containing the entire underlying vector
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [CowArcProfileEntry<'a>] {
        &mut self.inner[..]
    }

    /// Consum the `ProfileStream` and retrun the underlying vector
    #[inline]
    pub fn into_inner(self) -> Vec<CowArcProfileEntry<'a>> {
        self.inner
    }
}
impl<'a> AsMut<Vec<CowArcProfileEntry<'a>>> for ProfileStream<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<CowArcProfileEntry<'a>> {
        &mut self.inner
    }
}
impl<'a> AsMut<[CowArcProfileEntry<'a>]> for ProfileStream<'a> {
    #[inline]
    fn as_mut(&mut self) -> &mut [CowArcProfileEntry<'a>] {
        &mut self.inner[..]
    }
}
impl<'a> AsRef<Vec<CowArcProfileEntry<'a>>> for ProfileStream<'a> {
    #[inline]
    fn as_ref(&self) -> &Vec<CowArcProfileEntry<'a>> {
        &self.inner
    }
}
impl<'a> AsRef<[CowArcProfileEntry<'a>]> for ProfileStream<'a> {
    #[inline]
    fn as_ref(&self) -> &[CowArcProfileEntry<'a>] {
        &self.inner[..]
    }
}
impl<'a> Borrow<Vec<CowArcProfileEntry<'a>>> for ProfileStream<'a> {
    #[inline]
    fn borrow(&self) -> &Vec<CowArcProfileEntry<'a>> {
        &self.inner
    }
}
impl<'a> Borrow<[CowArcProfileEntry<'a>]> for ProfileStream<'a> {
    #[inline]
    fn borrow(&self) -> &[CowArcProfileEntry<'a>] {
        &self.inner[..]
    }
}
impl<'a> BorrowMut<Vec<CowArcProfileEntry<'a>>> for ProfileStream<'a> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Vec<CowArcProfileEntry<'a>> {
        &mut self.inner
    }
}
impl<'a> BorrowMut<[CowArcProfileEntry<'a>]> for ProfileStream<'a> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [CowArcProfileEntry<'a>] {
        &mut self.inner[..]
    }
}
impl fmt::Display for ProfileStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for profile_entry in &self.inner {
            write!(f, "{}", profile_entry)?;
        }
        Ok(())
    }
}
impl<'a> Extend<CowArcProfileEntry<'a>> for ProfileStream<'a> {
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = CowArcProfileEntry<'a>>,
    {
        self.inner.extend(iter);
    }
}
impl<'a> FromIterator<CowArcProfileEntry<'a>> for ProfileStream<'a> {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = CowArcProfileEntry<'a>>,
    {
        Self {
            inner: Vec::from_iter(iter),
        }
    }
}
impl<'a> IntoIterator for ProfileStream<'a> {
    type Item = CowArcProfileEntry<'a>;
    type IntoIter = vec::IntoIter<CowArcProfileEntry<'a>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl<'a> IntoIterator for &'a ProfileStream<'a> {
    type Item = &'a CowArcProfileEntry<'a>;
    type IntoIter = slice::Iter<'a, CowArcProfileEntry<'a>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
impl FromStr for ProfileStream<'_> {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self, Self> {
        let mut vec = Vec::new();
        let mut ret_as_err = false;
        for line in s.lines() {
            vec.push(CowArcProfileEntry::Owned(Arc::new(match line.parse() {
                Ok(profile_entry) => profile_entry,
                Err(invalid_profile_entry) => {
                    ret_as_err = true;
                    invalid_profile_entry
                }
            })));
        }

        if ret_as_err {
            Err(Self { inner: vec })
        } else {
            Ok(Self { inner: vec })
        }
    }
}

//
// ProfileEntry
//

/// Unwrap a value if it is `Ok(T)` or `return Err(ProfileEntry::_Invalid($line.to_string())`
///
/// # Examples
///
/// ```
/// fn make_frog() -> Result<Frog, Error> {
///     let mut tadpole = Tadpole::new();
///     for line in GUIDE {
///         tadpole.push(unwrap_or_invalid!(apply(line));
///     }
///     Frog::from(tadpole)
/// }
/// ```
macro_rules! unwrap_or_invalid {
    ($res:expr, $line:ident) => {
        match $res {
            Ok(ok_val) => ok_val,
            Err(_) => return Err(Self::_Invalid($line.to_string())),
        }
    };
}

/// A single entry (line) in a profile
#[non_exhaustive]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ProfileEntry {
    Apparmor,
    Blacklist(String),
    /// A empty line
    Blank,
    CapsDropAll,
    /// A comment (without the leading `#`)
    /// This variant might change in the future to something like `Comment(Comment::Foo(String))`
    Comment(String),
    DBusUser(DBusPolicy),
    DBusUserOwn(String),
    DBusUserTalk(String),
    DBusSystem(DBusPolicy),
    DisableMnt,
    Ignore(String),
    Include(String),
    IpcNamespace,
    JoinOrStart(String),
    MachineId,
    MemoryDenyWriteExecute,
    Mkdir(String),
    Mkfile(String),
    Netfilter,
    NetNone,
    No3d,
    Noblacklist(String),
    Nodvd,
    Nogroups,
    Nonewprivs,
    Noroot,
    Nosound,
    Notv,
    Nou2f,
    Novideo,
    /// `Private(None)`: `private`<br>
    /// `Private(Some(String::from("${HOME}/spam")))`: `private ${HOME}/spam`
    Private(Option<String>),
    PrivateBin(Vec<String>),
    PrivateDev,
    PrivateEtc(Vec<String>),
    PrivateTmp,
    Protocol(Protocols),
    ReadOnly(String),
    ReadWrite(String),
    /// `Seccomp(None)`: `seccomp`<br>
    /// `Seccomp(Some(vec!["!chroot".to_string()]))`: `seccomp !chroot`
    Seccomp(Option<Vec<String>>),
    ShellNone,
    Tracelog,
    Whitelist(String),
    WritableRunUser,
    /// An invalid line
    _Invalid(String),
    /// A unknow line, likely not implemented yet.
    /// This variant will be removed in the future.
    _Unknow(String),
}
impl ProfileEntry {
    pub fn is_comment(&self) -> bool {
        match self {
            Self::Comment(_) => true,
            _ => false,
        }
    }
}
impl fmt::Display for ProfileEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ProfileEntry::*;
        match self {
            Apparmor => writeln!(f, "apparmor")?,
            Blacklist(path) => writeln!(f, "blacklist {}", path)?,
            Blank => writeln!(f)?,
            CapsDropAll => writeln!(f, "caps.drop all")?,
            Comment(comment) => writeln!(f, "#{}", comment)?,
            DBusUser(policy) => writeln!(f, "dbus-user {}", policy)?,
            DBusUserOwn(name) => writeln!(f, "dbus-user.own {}", name)?,
            DBusUserTalk(name) => writeln!(f, "dbus-user.talk {}", name)?,
            DBusSystem(policy) => writeln!(f, "dbus-system {}", policy)?,
            DisableMnt => writeln!(f, "disable-mnt")?,
            Ignore(profile_line) => writeln!(f, "ignore {}", profile_line)?,
            Include(profile) => writeln!(f, "include {}", profile)?,
            IpcNamespace => writeln!(f, "ipc-namespace")?,
            JoinOrStart(name) => writeln!(f, "join-or-start {}", name)?,
            MachineId => writeln!(f, "machine-id")?,
            MemoryDenyWriteExecute => writeln!(f, "memory-deny-write-execute")?,
            Mkdir(path) => writeln!(f, "mkdir {}", path)?,
            Mkfile(path) => writeln!(f, "mkfile {}", path)?,
            Netfilter => writeln!(f, "netfilter")?,
            NetNone => writeln!(f, "net none")?,
            No3d => writeln!(f, "no3d")?,
            Noblacklist(path) => writeln!(f, "noblacklist {}", path)?,
            Nodvd => writeln!(f, "nodvd")?,
            Nogroups => writeln!(f, "nogroups")?,
            Nonewprivs => writeln!(f, "nonewprivs")?,
            Noroot => writeln!(f, "noroot")?,
            Nosound => writeln!(f, "nosound")?,
            Notv => writeln!(f, "notv")?,
            Nou2f => writeln!(f, "nou2f")?,
            Novideo => writeln!(f, "novideo")?,
            Private(None) => writeln!(f, "private")?,
            Private(Some(path)) => writeln!(f, "private {}", path)?,
            PrivateBin(bins) => writeln!(f, "private-bin {}", bins.join(","))?,
            PrivateDev => writeln!(f, "private-dev")?,
            PrivateEtc(files) => writeln!(f, "private-etc {}", files.join(","))?,
            PrivateTmp => writeln!(f, "private-tmp")?,
            Protocol(protocols) => writeln!(f, "protocol {}", protocols)?,
            ReadOnly(path) => writeln!(f, "read-only {}", path)?,
            ReadWrite(path) => writeln!(f, "read-write {}", path)?,
            Seccomp(None) => writeln!(f, "seccomp")?,
            Seccomp(Some(syscalls)) => writeln!(f, "seccomp {}", syscalls.join(","))?,
            ShellNone => writeln!(f, "shell none")?,
            Tracelog => writeln!(f, "tracelog")?,
            Whitelist(path) => writeln!(f, "whitelist {}", path)?,
            WritableRunUser => writeln!(f, "writable-run-user")?,
            _Invalid(_line) => unimplemented!(), // writeln!(f, "#INVALID!{}", line)?,
            _Unknow(profile_line) => writeln!(f, "{}", profile_line)?,
            //_ => unimplemented!(),
        }
        Ok(())
    }
}
impl FromStr for ProfileEntry {
    type Err = Self;

    /// Parses a string `line` to return a value of this type.
    ///
    /// # Errors
    ///
    /// `ProfileEntry::_Invalid(line)`
    fn from_str(line: &str) -> Result<Self, Self> {
        use ProfileEntry::*;

        Ok(if line == "apparmor" {
            Apparmor
        } else if line.starts_with("blacklist ") {
            Blacklist(line[10..].to_string())
        } else if line == "" {
            Blank
        } else if line == "caps.drop all" {
            CapsDropAll
        } else if line.starts_with('#') {
            Comment(line[1..].to_string())
        } else if line == "dbus-user filter" {
            DBusUser(DBusPolicy::Filter)
        } else if line == "dbus-user none" {
            DBusUser(DBusPolicy::None)
        } else if line.starts_with("dbus-user.own ") {
            DBusUserOwn(line[14..].to_string())
        } else if line.starts_with("dbus-user.talk ") {
            DBusUserTalk(line[15..].to_string())
        } else if line == "dbus-system filter" {
            DBusSystem(DBusPolicy::Filter)
        } else if line == "dbus-system none" {
            DBusSystem(DBusPolicy::None)
        } else if line == "disable-mnt" {
            DisableMnt
        } else if line.starts_with("ignore ") {
            Ignore(line[7..].to_string())
        } else if line.starts_with("include ") {
            Include(line[8..].to_string())
        } else if line == "ipc-namespace" {
            IpcNamespace
        } else if line.starts_with("join-or-start ") {
            JoinOrStart(line[14..].to_string())
        } else if line == "machine-id" {
            MachineId
        } else if line == "memory-deny-write-execute" {
            MemoryDenyWriteExecute
        } else if line.starts_with("mkdir ") {
            Mkdir(line[6..].to_string())
        } else if line.starts_with("mkfile ") {
            Mkfile(line[7..].to_string())
        } else if line == "netfilter" {
            Netfilter
        } else if line == "net none" {
            NetNone
        } else if line == "no3d" {
            No3d
        } else if line.starts_with("noblacklist ") {
            Noblacklist(line[12..].to_string())
        } else if line == "nodvd" {
            Nodvd
        } else if line == "nogroups" {
            Nogroups
        } else if line == "nonewprivs" {
            Nonewprivs
        } else if line == "noroot" {
            Noroot
        } else if line == "nosound" {
            Nosound
        } else if line == "notv" {
            Notv
        } else if line == "nou2f" {
            Nou2f
        } else if line == "novideo" {
            Novideo
        } else if line == "private" {
            Private(None)
        } else if line.starts_with("private ") {
            Private(Some(line[8..].to_string()))
        } else if line.starts_with("private-bin ") {
            PrivateBin(line[12..].split(',').map(String::from).collect())
        } else if line == "private-dev" {
            PrivateDev
        } else if line.starts_with("private-etc ") {
            PrivateEtc(line[12..].split(',').map(String::from).collect())
        } else if line == "private-tmp" {
            PrivateTmp
        } else if line.starts_with("protocol ") {
            Protocol(unwrap_or_invalid!(
                Protocols::try_from(&*line[9..].split(',').collect::<Vec<_>>()),
                line
            ))
        } else if line.starts_with("read-only ") {
            ReadOnly(line[12..].to_string())
        } else if line.starts_with("read-write ") {
            ReadWrite(line[13..].to_string())
        } else if line == "seccomp" {
            Seccomp(None)
        } else if line.starts_with("seccomp ") {
            Seccomp(Some(line[8..].split(',').map(String::from).collect()))
        } else if line == "shell none" {
            ShellNone
        } else if line == "tracelog" {
            Tracelog
        } else if line.starts_with("whitelist ") {
            Whitelist(line[10..].to_string())
        } else if line == "writable-run-user" {
            WritableRunUser
        } else {
            _Unknow(line.to_string())
        })
    }
}
impl PartialEq<CowArcProfileEntry<'_>> for ProfileEntry {
    #[inline]
    fn eq(&self, other: &CowArcProfileEntry) -> bool {
        PartialEq::eq(self, &***other)
    }
}
impl PartialEq<ProfileEntry> for CowArcProfileEntry<'_> {
    #[inline]
    fn eq(&self, other: &ProfileEntry) -> bool {
        PartialEq::eq(&***self, other)
    }
}
impl Borrow<ProfileEntry> for CowArcProfileEntry<'_> {
    #[inline]
    fn borrow(&self) -> &ProfileEntry {
        &***self
    }
}

//
// Protocols
//

bitflags! {
    /// Protocols from firejails `protocol` option
    pub struct Protocols: u8 {
        const UNIX    = 0b00000001;
        const INET    = 0b00000010;
        const INET6   = 0b00000100;
        const NETLINK = 0b00001000;
        const PACKET  = 0b00010000;
    }
}
/// Create a new `Protocols` instance from a slice of `str`s
///
/// # Examples
///
/// ```
/// # use std::convert::TryFrom;
/// assert_eq!(
///     Protocols::try_from(&["unix", "inet", "inet6"])?,
///     Protocols::UNIX | Protocols::INET | Protocols::INET6,
/// );
/// ```
///
/// ```should_panic
/// # use std::convert::TryFrom;
/// Protocols::try_from(&["invalid"]).unwrap(); // This will fail!
/// ```
impl TryFrom<&[&str]> for Protocols {
    type Error = anyhow::Error;

    /// Performs the conversion.
    ///
    /// # Errors
    ///
    /// `anyhow::anyhow!("This is not a valid protocol")`
    fn try_from(protos: &[&str]) -> Result<Self, anyhow::Error> {
        let mut protocols = Self::empty();
        for proto in protos {
            protocols.insert(match *proto {
                "unix" => Self::UNIX,
                "inet" => Self::INET,
                "inet6" => Self::INET6,
                "netlink" => Self::NETLINK,
                "packet" => Self::PACKET,
                _ => bail!("This is not a valid protocol"),
            });
        }
        Ok(protocols)
    }
}
impl fmt::Display for Protocols {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            if self.is_empty() {
                panic!();
            } else if *self == Self::UNIX {
                "protocol unix"
            } else if *self == Self::UNIX | Self::INET {
                "protocol unix,inet"
            } else if *self == Self::UNIX | Self::INET | Self::INET6 {
                "protocol unix,inet,inet6"
            } else if *self == Self::UNIX | Self::INET | Self::INET6 | Self::NETLINK {
                "protocol unix,inet,inet6,netlink"
            } else if *self == Self::UNIX | Self::INET | Self::INET6 | Self::NETLINK | Self::PACKET {
                "protocol unix,inet,inet6,netlink,packet"
            } else if *self == Self::UNIX | Self::INET | Self::INET6 | Self::PACKET {
                "protocol unix,inet,inet6,packet"
            } else if *self == Self::UNIX | Self::INET | Self::NETLINK {
                "protocol unix,inet,netlink"
            } else if *self == Self::UNIX | Self::INET | Self::NETLINK | Self::PACKET {
                "protocol unix,inet,netlink,packet"
            } else if *self == Self::UNIX | Self::INET | Self::PACKET {
                "protocol unix,inet,packet"
            } else if *self == Self::UNIX | Self::INET6 {
                "protocol unix,inet6"
            } else if *self == Self::UNIX | Self::INET6 | Self::NETLINK {
                "protocol unix,inet6,netlink"
            } else if *self == Self::UNIX | Self::INET6 | Self::NETLINK | Self::PACKET {
                "protocol unix,inet6,netlink,packet"
            } else if *self == Self::UNIX | Self::INET6 | Self::PACKET {
                "protocol unix,inet6,packet"
            } else if *self == Self::UNIX | Self::NETLINK {
                "protocol unix,netlink"
            } else if *self == Self::UNIX | Self::NETLINK | Self::PACKET {
                "protocol unix,netlink,packet"
            } else if *self == Self::UNIX | Self::PACKET {
                "protocol unix,packet"
            } else if *self == Self::INET {
                "protocol inet"
            } else if *self == Self::INET | Self::INET6 {
                "protocol inet,inet6"
            } else if *self == Self::INET | Self::INET6 | Self::NETLINK {
                "protocol inet,inet6,netlink"
            } else if *self == Self::INET | Self::INET6 | Self::NETLINK | Self::PACKET {
                "protocol inet,inet6,netlink,packet"
            } else if *self == Self::INET | Self::INET6 | Self::PACKET {
                "protocol inet,inet6,packet"
            } else if *self == Self::INET6 {
                "protocol inet6"
            } else if *self == Self::INET6 | Self::NETLINK {
                "protocol inet6,netlink"
            } else if *self == Self::INET6 | Self::NETLINK | Self::PACKET {
                "protocol inet6,netlink,packet"
            } else if *self == Self::INET6 | Self::PACKET {
                "protocol inet6,packet"
            } else if *self == Self::NETLINK {
                "protocol netlink"
            } else if *self == Self::NETLINK | Self::PACKET {
                "protocol netlink,packet"
            } else if *self == Self::PACKET {
                "protocol packet"
            } else {
                unreachable!();
            }
        )
    }
}

//
// DBusPolicy
//

/// DBus Policy
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum DBusPolicy {
    Allow,
    Filter,
    None,
}
impl fmt::Display for DBusPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => unimplemented!("fmt::Display for DBusPolicy::Allow is not implemented yet. It does not exists in profiles."),
            Self::Filter => write!(f, "filter"),
            Self::None => write!(f, "none"),
        }
    }
}
