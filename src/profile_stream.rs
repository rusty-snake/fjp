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

use crate::utils::{join, IteratorExt};
use anyhow::anyhow;
use std::borrow::{Borrow, BorrowMut, Cow};
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
    AllowDebuggers,
    Allusers,
    Apparmor,
    Blacklist(String),
    /// A empty line
    Blank,
    Caps,
    CapsDropAll,
    CapsDrop(Vec<Capabilities>),
    CapsKeep(Vec<Capabilities>),
    /// A comment (without the leading `#`)
    /// This variant might change in the future to something like `Comment(Comment::Foo(String))`
    Comment(String),
    DBusUser(DBusPolicy),
    DBusUserOwn(String),
    DBusUserTalk(String),
    DBusSystem(DBusPolicy),
    DBusSystemOwn(String),
    DBusSystemTalk(String),
    DisableMnt,
    Hostname(String),
    Ignore(String),
    Include(String),
    IpcNamespace,
    JoinOrStart(String),
    MachineId,
    MemoryDenyWriteExecute,
    Mkdir(String),
    Mkfile(String),
    Name(String),
    Netfilter,
    NetNone,
    No3d,
    Noblacklist(String),
    Nodvd,
    Noexec(String),
    Nogroups,
    Nonewprivs,
    Noroot,
    Nosound,
    Notv,
    Nou2f,
    Novideo,
    Nowhitelist(String),
    /// `Private(None)`: `private`<br>
    /// `Private(Some(String::from("${HOME}/spam")))`: `private ${HOME}/spam`
    Private(Option<String>),
    PrivateBin(Vec<String>),
    PrivateCache,
    PrivateCwd(String),
    PrivateDev,
    PrivateEtc(Vec<String>),
    PrivateLib(Option<Vec<String>>),
    PrivateOpt(Vec<String>),
    PrivateSrv(Vec<String>),
    PrivateTmp,
    Protocol(Vec<Protocol>),
    Quiet,
    ReadOnly(String),
    ReadWrite(String),
    /// `Seccomp(None)`: `seccomp`<br>
    /// `Seccomp(Some(vec!["!chroot".to_string()]))`: `seccomp !chroot`
    Seccomp(Option<Vec<String>>),
    SeccompBlockSecondary,
    SeccompDrop(Vec<String>),
    ShellNone,
    Tracelog,
    Whitelist(String),
    WritableRunUser,
    WritableVar,
    WritableVarLog,
    X11None,
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
            AllowDebuggers => writeln!(f, "allow-debuggers")?,
            Allusers => writeln!(f, "allusers")?,
            Apparmor => writeln!(f, "apparmor")?,
            Blacklist(path) => writeln!(f, "blacklist {}", path)?,
            Blank => writeln!(f)?,
            Caps => writeln!(f, "caps")?,
            CapsDropAll => writeln!(f, "caps.drop all")?,
            CapsDrop(caps) => writeln!(f, "caps.drop {}", join(',', caps))?,
            CapsKeep(caps) => writeln!(f, "caps.keep {}", join(',', caps))?,
            Comment(comment) => writeln!(f, "#{}", comment)?,
            DBusUser(policy) => writeln!(f, "dbus-user {}", policy)?,
            DBusUserOwn(name) => writeln!(f, "dbus-user.own {}", name)?,
            DBusUserTalk(name) => writeln!(f, "dbus-user.talk {}", name)?,
            DBusSystem(policy) => writeln!(f, "dbus-system {}", policy)?,
            DBusSystemOwn(name) => writeln!(f, "dbus-system.own {}", name)?,
            DBusSystemTalk(name) => writeln!(f, "dbus-system.talk {}", name)?,
            DisableMnt => writeln!(f, "disable-mnt")?,
            Hostname(hostname) => writeln!(f, "hostname {}", hostname)?,
            Ignore(profile_line) => writeln!(f, "ignore {}", profile_line)?,
            Include(profile) => writeln!(f, "include {}", profile)?,
            IpcNamespace => writeln!(f, "ipc-namespace")?,
            JoinOrStart(name) => writeln!(f, "join-or-start {}", name)?,
            MachineId => writeln!(f, "machine-id")?,
            MemoryDenyWriteExecute => writeln!(f, "memory-deny-write-execute")?,
            Mkdir(path) => writeln!(f, "mkdir {}", path)?,
            Mkfile(path) => writeln!(f, "mkfile {}", path)?,
            Name(name) => writeln!(f, "name {}", name)?,
            Netfilter => writeln!(f, "netfilter")?,
            NetNone => writeln!(f, "net none")?,
            No3d => writeln!(f, "no3d")?,
            Noblacklist(path) => writeln!(f, "noblacklist {}", path)?,
            Nodvd => writeln!(f, "nodvd")?,
            Noexec(path) => writeln!(f, "noexec {}", path)?,
            Nogroups => writeln!(f, "nogroups")?,
            Nonewprivs => writeln!(f, "nonewprivs")?,
            Noroot => writeln!(f, "noroot")?,
            Nosound => writeln!(f, "nosound")?,
            Notv => writeln!(f, "notv")?,
            Nou2f => writeln!(f, "nou2f")?,
            Novideo => writeln!(f, "novideo")?,
            Nowhitelist(path) => writeln!(f, "nowhitelist {}", path)?,
            Private(None) => writeln!(f, "private")?,
            Private(Some(path)) => writeln!(f, "private {}", path)?,
            PrivateBin(bins) => writeln!(f, "private-bin {}", bins.join(","))?,
            PrivateCache => writeln!(f, "private-cache")?,
            PrivateCwd(path) => writeln!(f, "private-cwd {}", path)?,
            PrivateDev => writeln!(f, "private-dev")?,
            PrivateEtc(files) => writeln!(f, "private-etc {}", files.join(","))?,
            PrivateLib(None) => writeln!(f, "private-lib")?,
            PrivateLib(Some(files)) => writeln!(f, "private-lib {}", files.join(","))?,
            PrivateOpt(files) => writeln!(f, "private-opt {}", files.join(","))?,
            PrivateSrv(files) => writeln!(f, "private-srv {}", files.join(","))?,
            PrivateTmp => writeln!(f, "private-tmp")?,
            Protocol(protocols) => writeln!(f, "protocol {}", join(",", protocols))?,
            Quiet => writeln!(f, "quiet")?,
            ReadOnly(path) => writeln!(f, "read-only {}", path)?,
            ReadWrite(path) => writeln!(f, "read-write {}", path)?,
            Seccomp(None) => writeln!(f, "seccomp")?,
            Seccomp(Some(syscalls)) => writeln!(f, "seccomp {}", syscalls.join(","))?,
            SeccompBlockSecondary => writeln!(f, "seccomp.block-secondary")?,
            SeccompDrop(syscalls) => writeln!(f, "seccomp.drop {}", syscalls.join(","))?,
            ShellNone => writeln!(f, "shell none")?,
            Tracelog => writeln!(f, "tracelog")?,
            Whitelist(path) => writeln!(f, "whitelist {}", path)?,
            WritableRunUser => writeln!(f, "writable-run-user")?,
            WritableVar => writeln!(f, "writable-var")?,
            WritableVarLog => writeln!(f, "writable-var-log")?,
            X11None => writeln!(f, "x11 none")?,
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

        Ok(if line == "allow-debuggers" {
            AllowDebuggers
        } else if line == "allusers" {
            Allusers
        } else if line == "apparmor" {
            Apparmor
        } else if line.starts_with("blacklist ") {
            Blacklist(line[10..].to_string())
        } else if line == "" {
            Blank
        } else if line == "caps" {
            Caps
        } else if line == "caps.drop all" {
            CapsDropAll
        } else if line.starts_with("caps.drop ") {
            CapsDrop(unwrap_or_invalid!(
                line[10..].split(',').map(str::parse).collect(),
                line
            ))
        } else if line.starts_with("caps.keep ") {
            CapsKeep(unwrap_or_invalid!(
                line[10..].split(',').map(str::parse).collect(),
                line
            ))
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
        } else if line.starts_with("dbus-system.own ") {
            DBusSystemOwn(line[16..].to_string())
        } else if line.starts_with("dbus-system.talk ") {
            DBusSystemTalk(line[17..].to_string())
        } else if line == "disable-mnt" {
            DisableMnt
        } else if line.starts_with("hostname ") {
            Hostname(line[9..].to_string())
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
        } else if line.starts_with("name ") {
            Name(line[5..].to_string())
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
        } else if line.starts_with("noexec ") {
            Noexec(line[7..].to_string())
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
        } else if line.starts_with("nowhitelist ") {
            Nowhitelist(line[12..].to_string())
        } else if line == "private" {
            Private(None)
        } else if line.starts_with("private ") {
            Private(Some(line[8..].to_string()))
        } else if line.starts_with("private-bin ") {
            PrivateBin(line[12..].split(',').map(String::from).collect())
        } else if line == "private-cache" {
            PrivateCache
        } else if line.starts_with("private-cwd ") {
            PrivateCwd(line[12..].to_string())
        } else if line == "private-dev" {
            PrivateDev
        } else if line.starts_with("private-etc ") {
            PrivateEtc(line[12..].split(',').map(String::from).collect())
        } else if line == "private-lib" {
            PrivateLib(None)
        } else if line.starts_with("private-lib ") {
            PrivateLib(Some(line[12..].split(',').map(String::from).collect()))
        } else if line.starts_with("private-opt ") {
            PrivateOpt(line[12..].split(',').map(String::from).collect())
        } else if line.starts_with("private-srv ") {
            PrivateSrv(line[12..].split(',').map(String::from).collect())
        } else if line == "private-tmp" {
            PrivateTmp
        } else if line.starts_with("protocol ") {
            Protocol(unwrap_or_invalid!(
                line[9..]
                    .split(',')
                    .map(FromStr::from_str)
                    .collect_results_to_vec(),
                line
            ))
        } else if line == "quiet" {
            Quiet
        } else if line.starts_with("read-only ") {
            ReadOnly(line[12..].to_string())
        } else if line.starts_with("read-write ") {
            ReadWrite(line[13..].to_string())
        } else if line == "seccomp" {
            Seccomp(None)
        } else if line.starts_with("seccomp ") {
            Seccomp(Some(line[8..].split(',').map(String::from).collect()))
        } else if line == "seccomp.block-secondary" {
            SeccompBlockSecondary
        } else if line.starts_with("seccomp.drop ") {
            SeccompDrop(line[13..].split(',').map(String::from).collect())
        } else if line == "shell none" {
            ShellNone
        } else if line == "tracelog" {
            Tracelog
        } else if line.starts_with("whitelist ") {
            Whitelist(line[10..].to_string())
        } else if line == "writable-run-user" {
            WritableRunUser
        } else if line == "writable-var" {
            WritableVar
        } else if line == "writable-var-log" {
            WritableVarLog
        } else if line == "x11 none" {
            X11None
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
// Protocol
//

/// A `Protocol` from firejails `protocol` command
// TODO: PartialOrd
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Protocol {
    Unix,
    Inet,
    Inet6,
    Netlink,
    Packet,
}
/// Create a new `Protocol` instance from `str`
///
/// # Examples
///
/// ```
/// assert_eq!(
///     Protocol::from_str("unix")?,
///     Protocol::Unix,
/// );
/// ```
///
/// ```should_panic
/// "invalid".parse::<Protocol>().unwrap(); // This will fail!
/// ```
impl FromStr for Protocol {
    type Err = anyhow::Error;

    /// Parses a str to a Protocol
    ///
    /// # Errors
    ///
    /// `anyhow::anyhow!("This is not a valid protocol")`
    fn from_str(proto: &str) -> Result<Self, anyhow::Error> {
        match proto {
            "unix" => Ok(Self::Unix),
            "inet" => Ok(Self::Inet),
            "inet6" => Ok(Self::Inet6),
            "netlink" => Ok(Self::Netlink),
            "packet" => Ok(Self::Packet),
            _ => Err(anyhow!("This is not a valid protocol")),
        }
    }
}
impl fmt::Display for Protocol {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Self::Unix => "unix",
                Self::Inet => "inet",
                Self::Inet6 => "inet6",
                Self::Netlink => "netlink",
                Self::Packet => "packet",
            },
        )
    }
}

//
// Capabilities
//

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Capabilities {
    AuditControl,
    AuditRead,
    AuditWrite,
    BlockSuspend,
    Chown,
    DacOverride,
    DacReadSearch,
    Fowner,
    Fsetid,
    IpcLock,
    IpcOwner,
    Kill,
    Lease,
    LinuxImmutable,
    MacAdmin,
    MacOverride,
    Mknod,
    NetAdmin,
    NetBindService,
    NetBroadcast,
    NetRaw,
    Setfcap,
    Setgid,
    Setpcap,
    Setuid,
    SysAdmin,
    SysBoot,
    SysChroot,
    SysModule,
    SysNice,
    SysPacct,
    SysPtrace,
    SysRawio,
    SysResource,
    SysTime,
    SysTtyConfig,
    Syslog,
    WakeAlarm,
}
impl fmt::Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Capabilities::*;
        match self {
            AuditControl => write!(f, "audit_control")?,
            AuditRead => write!(f, "audit_read")?,
            AuditWrite => write!(f, "audit_write")?,
            BlockSuspend => write!(f, "block_suspend")?,
            Chown => write!(f, "chown")?,
            DacOverride => write!(f, "dac_override")?,
            DacReadSearch => write!(f, "dac_read_search")?,
            Fowner => write!(f, "fowner")?,
            Fsetid => write!(f, "fsetid")?,
            IpcLock => write!(f, "ipc_lock")?,
            IpcOwner => write!(f, "ipc_owner")?,
            Kill => write!(f, "kill")?,
            Lease => write!(f, "lease")?,
            LinuxImmutable => write!(f, "linux_immutable")?,
            MacAdmin => write!(f, "mac_admin")?,
            MacOverride => write!(f, "mac_override")?,
            Mknod => write!(f, "mknod")?,
            NetAdmin => write!(f, "net_admin")?,
            NetBindService => write!(f, "net_bind_service")?,
            NetBroadcast => write!(f, "net_broadcast")?,
            NetRaw => write!(f, "net_raw")?,
            Setfcap => write!(f, "setfcap")?,
            Setgid => write!(f, "setgid")?,
            Setpcap => write!(f, "setpcap")?,
            Setuid => write!(f, "setuid")?,
            SysAdmin => write!(f, "sys_admin")?,
            SysBoot => write!(f, "sys_boot")?,
            SysChroot => write!(f, "sys_chroot")?,
            SysModule => write!(f, "sys_module")?,
            SysNice => write!(f, "sys_nice")?,
            SysPacct => write!(f, "sys_pacct")?,
            SysPtrace => write!(f, "sys_ptrace")?,
            SysRawio => write!(f, "sys_rawio")?,
            SysResource => write!(f, "sys_resource")?,
            SysTime => write!(f, "sys_time")?,
            SysTtyConfig => write!(f, "sys_tty_config")?,
            Syslog => write!(f, "syslog")?,
            WakeAlarm => write!(f, "wake_alarm")?,
        }
        Ok(())
    }
}
impl FromStr for Capabilities {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        use Capabilities::*;
        match s {
            "audit_control" => Ok(AuditControl),
            "audit_read" => Ok(AuditRead),
            "audit_write" => Ok(AuditWrite),
            "block_suspend" => Ok(BlockSuspend),
            "chown" => Ok(Chown),
            "dac_override" => Ok(DacOverride),
            "dac_read_search" => Ok(DacReadSearch),
            "fowner" => Ok(Fowner),
            "fsetid" => Ok(Fsetid),
            "ipc_lock" => Ok(IpcLock),
            "ipc_owner" => Ok(IpcOwner),
            "kill" => Ok(Kill),
            "lease" => Ok(Lease),
            "linux_immutable" => Ok(LinuxImmutable),
            "mac_admin" => Ok(MacAdmin),
            "mac_override" => Ok(MacOverride),
            "mknod" => Ok(Mknod),
            "net_admin" => Ok(NetAdmin),
            "net_bind_service" => Ok(NetBindService),
            "net_broadcast" => Ok(NetBroadcast),
            "net_raw" => Ok(NetRaw),
            "setfcap" => Ok(Setfcap),
            "setgid" => Ok(Setgid),
            "setpcap" => Ok(Setpcap),
            "setuid" => Ok(Setuid),
            "sys_admin" => Ok(SysAdmin),
            "sys_boot" => Ok(SysBoot),
            "sys_chroot" => Ok(SysChroot),
            "sys_module" => Ok(SysModule),
            "sys_nice" => Ok(SysNice),
            "sys_pacct" => Ok(SysPacct),
            "sys_ptrace" => Ok(SysPtrace),
            "sys_rawio" => Ok(SysRawio),
            "sys_resource" => Ok(SysResource),
            "sys_time" => Ok(SysTime),
            "sys_tty_config" => Ok(SysTtyConfig),
            "syslog" => Ok(Syslog),
            "wake_alarm" => Ok(WakeAlarm),
            _ => Err(anyhow!("Unknow cap: {}", s)),
        }
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
