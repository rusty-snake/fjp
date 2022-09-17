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

//! Abstract representations of a firejail profile

#![allow(clippy::cognitive_complexity)]

use crate::utils::join;
use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::iter::FromIterator;
use std::slice;
use std::str::FromStr;
use std::sync::Arc;
use std::vec;

/// An abstract stream of lines in a firejail profile
#[derive(Clone, Debug)]
pub struct ProfileStream {
    inner: Vec<Line>,
}
impl ProfileStream {
    /// Check whether `self` contains `content` or not
    pub fn contains(&self, content: &Content) -> bool {
        self.inner.iter().any(|l| &*l.content == content)
    }

    /// Check whether there are any invalid lines
    pub fn has_errors(&self) -> bool {
        self.inner.iter().any(|line| !line.is_valid())
    }

    /// Retruns a ProfileStream containing all invalid lines from self
    pub fn errors(&self) -> Self {
        self.inner
            .iter()
            .filter(|line| !line.is_valid())
            .cloned()
            .collect()
    }

    /// Set all `lineno` in the `ProfileStream` to `None`
    pub fn strip_lineno(&mut self) {
        for l in &mut self.inner {
            l.lineno = None;
        }
    }

    /// Rewrite all `lineno` based on the current position in the `ProfileStream`
    pub fn rewrite_lineno(&mut self) {
        for (i, l) in self.inner.iter_mut().enumerate() {
            l.lineno = Some(i);
        }
    }

    /// Returns `true` if the profile-stream contains no lines
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
impl ProfileStream {
    /// Extracts a slice containing the entire underlying vector
    #[inline]
    pub fn as_slice(&self) -> &[Line] {
        &self.inner[..]
    }

    /// Extracts a mutable slice containing the entire underlying vector
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [Line] {
        &mut self.inner[..]
    }

    /// Consum the `ProfileStream` and retrun the underlying vector
    #[inline]
    pub fn into_inner(self) -> Vec<Line> {
        self.inner
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, Line> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, Line> {
        self.inner.iter_mut()
    }
}
macro_rules! impl_borrow_and_convert_traits {
    ( $( $trait_:ty = fn $fname:ident$params:tt -> $rt:ty : $body:expr; )* ) => {
        $(
            impl $trait_ for ProfileStream {
                #[inline]
                fn $fname$params -> $rt {
                    $body
                }
            }
        )*
    };
}
impl_borrow_and_convert_traits! {
    AsMut<Vec<Line>>     = fn as_mut(&mut self)     -> &mut Vec<Line>: &mut self.inner    ;
    AsMut<[Line]>        = fn as_mut(&mut self)     -> &mut [Line]   : &mut self.inner[..];
    AsRef<Vec<Line>>     = fn as_ref(&self)         -> &Vec<Line>    : &self.inner        ;
    AsRef<[Line]>        = fn as_ref(&self)         -> &[Line]       : &self.inner[..]    ;
    BorrowMut<Vec<Line>> = fn borrow_mut(&mut self) -> &mut Vec<Line>: &mut self.inner    ;
    BorrowMut<[Line]>    = fn borrow_mut(&mut self) -> &mut [Line]   : &mut self.inner[..];
    Borrow<Vec<Line>>    = fn borrow(&self)         -> &Vec<Line>    : &self.inner        ;
    Borrow<[Line]>       = fn borrow(&self)         -> &[Line]       : &self.inner[..]    ;
    From<Vec<Line>>      = fn from(v: Vec<Line>)    -> Self          : Self { inner: v }  ;
}
impl From<ProfileStream> for Vec<Line> {
    fn from(ps: ProfileStream) -> Self {
        ps.inner
    }
}
impl Extend<Line> for ProfileStream {
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = Line>,
    {
        self.inner.extend(iter);
    }
}
impl FromIterator<Line> for ProfileStream {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Line>,
    {
        Self {
            inner: Vec::from_iter(iter),
        }
    }
}
impl IntoIterator for ProfileStream {
    type Item = Line;
    type IntoIter = vec::IntoIter<Line>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
impl<'a> IntoIterator for &'a ProfileStream {
    type Item = &'a Line;
    type IntoIter = slice::Iter<'a, Line>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}
impl fmt::Display for ProfileStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for Line { content, .. } in self.inner.iter().map(Borrow::borrow) {
            write!(f, "{}", content)?;
        }
        Ok(())
    }
}
impl FromStr for ProfileStream {
    type Err = Self;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut valid = true;
        let profile_stream = Self {
            inner: s
                .lines()
                .map(|line| {
                    line.parse::<Content>().unwrap_or_else(|invalid| {
                        valid = false;
                        invalid
                    })
                })
                .map(Arc::new)
                .enumerate()
                .map(|(lineno, content)| (Some(lineno), content))
                .map(|(lineno, content)| Line { lineno, content })
                .collect(),
        };

        if valid {
            Ok(profile_stream)
        } else {
            Err(profile_stream)
        }
    }
}

//
// Line
//

/// A profile-line
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// The line number of this line if known
    pub lineno: Option<usize>,
    /// The content of this line
    pub content: Arc<Content>,
}
impl Line {
    /// Returns `true` if this line is valid or `false` otherwise.
    ///
    /// A line is valid if `content` is something else than [`Content::Invalid`].
    pub fn is_valid(&self) -> bool {
        !matches!(*self.content, Content::Invalid(_, _))
    }

    /// Returns `true` if this line is a comment or `false` otherwise.
    pub fn is_comment(&self) -> bool {
        matches!(*self.content, Content::Comment(_))
    }
}
impl AsRef<Content> for Line {
    fn as_ref(&self) -> &Content {
        &*self.content
    }
}

//
// Content
//

/// The content of a profile-`Line`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Blank,
    Command(Command),
    Comment(String),
    Conditional(Conditional),
    Invalid(String, Error),
}
impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => writeln!(f),
            Self::Command(command) => writeln!(f, "{}", command),
            Self::Comment(comment) => writeln!(f, "#{}", comment),
            Self::Conditional(conditional) => writeln!(f, "{}", conditional),
            Self::Invalid(invalid, _) => writeln!(f, "{}", invalid),
        }
    }
}
impl FromStr for Content {
    type Err = Self;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if line.is_empty() {
            Ok(Self::Blank)
        } else if let Some(comment) = line.strip_prefix('#') {
            Ok(Self::Comment(comment.to_string()))
        } else if line.starts_with('?') {
            match line.parse() {
                Ok(cond) => Ok(Self::Conditional(cond)),
                Err(err) => Err(Self::Invalid(line.to_string(), err)),
            }
        } else {
            match line.parse() {
                Ok(comm) => Ok(Self::Command(comm)),
                Err(err) => Err(Self::Invalid(line.to_string(), err)),
            }
        }
    }
}

//
// Command
//

/// A firejail command
#[non_exhaustive]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Command {
    AllowDebuggers,
    Allusers,
    Apparmor,
    Bind(String, String),
    Blacklist(String),
    BlacklistNolog(String),
    Caps,
    CapsDropAll,
    CapsDrop(Vec<Capabilities>),
    CapsKeep(Vec<Capabilities>),
    DBusUser(DBusPolicy),
    DBusUserOwn(String),
    DBusUserTalk(String),
    DBusSystem(DBusPolicy),
    DBusSystemOwn(String),
    DBusSystemTalk(String),
    DisableMnt,
    /// `Env(String::from("WEBKIT_FORCE_SANDBOX"), String::from("0"))`: `env WEBKIT_FORCE_SANDBOX=0`
    Env(String, String),
    Hostname(String),
    Ignore(String),
    /// TODO: Recusive `ProfileStream`s
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
    Noinput,
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
    Rmenv(String),
    /// `Seccomp(None)`: `seccomp`<br>
    /// `Seccomp(Some(vec!["!chroot".to_string()]))`: `seccomp !chroot`
    Seccomp(Option<Vec<String>>),
    SeccompBlockSecondary,
    SeccompDrop(Vec<String>),
    SeccompErrorAction(SeccompErrorAction),
    ShellNone,
    Tmpfs(String),
    Tracelog,
    Whitelist(String),
    WriteableEtc,
    WritableRunUser,
    WritableVar,
    WritableVarLog,
    X11None,
}
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Command::*;
        match self {
            AllowDebuggers => write!(f, "allow-debuggers"),
            Allusers => write!(f, "allusers"),
            Apparmor => write!(f, "apparmor"),
            Bind(src_path, dst_path) => write!(f, "bind {},{}", src_path, dst_path),
            Blacklist(path) => write!(f, "blacklist {}", path),
            BlacklistNolog(path) => write!(f, "blacklist-nolog {}", path),
            Caps => write!(f, "caps"),
            CapsDropAll => write!(f, "caps.drop all"),
            CapsDrop(caps) => write!(f, "caps.drop {}", join(',', caps)),
            CapsKeep(caps) => write!(f, "caps.keep {}", join(',', caps)),
            DBusUser(policy) => write!(f, "dbus-user {}", policy),
            DBusUserOwn(name) => write!(f, "dbus-user.own {}", name),
            DBusUserTalk(name) => write!(f, "dbus-user.talk {}", name),
            DBusSystem(policy) => write!(f, "dbus-system {}", policy),
            DBusSystemOwn(name) => write!(f, "dbus-system.own {}", name),
            DBusSystemTalk(name) => write!(f, "dbus-system.talk {}", name),
            DisableMnt => write!(f, "disable-mnt"),
            Env(name, value) => write!(f, "env {}={}", name, value),
            Hostname(hostname) => write!(f, "hostname {}", hostname),
            Ignore(profile_line) => write!(f, "ignore {}", profile_line),
            Include(profile) => write!(f, "include {}", profile),
            IpcNamespace => write!(f, "ipc-namespace"),
            JoinOrStart(name) => write!(f, "join-or-start {}", name),
            MachineId => write!(f, "machine-id"),
            MemoryDenyWriteExecute => write!(f, "memory-deny-write-execute"),
            Mkdir(path) => write!(f, "mkdir {}", path),
            Mkfile(path) => write!(f, "mkfile {}", path),
            Name(name) => write!(f, "name {}", name),
            Netfilter => write!(f, "netfilter"),
            NetNone => write!(f, "net none"),
            No3d => write!(f, "no3d"),
            Noblacklist(path) => write!(f, "noblacklist {}", path),
            Nodvd => write!(f, "nodvd"),
            Noexec(path) => write!(f, "noexec {}", path),
            Nogroups => write!(f, "nogroups"),
            Noinput => write!(f, "noinput"),
            Nonewprivs => write!(f, "nonewprivs"),
            Noroot => write!(f, "noroot"),
            Nosound => write!(f, "nosound"),
            Notv => write!(f, "notv"),
            Nou2f => write!(f, "nou2f"),
            Novideo => write!(f, "novideo"),
            Nowhitelist(path) => write!(f, "nowhitelist {}", path),
            Private(None) => write!(f, "private"),
            Private(Some(path)) => write!(f, "private {}", path),
            PrivateBin(bins) => write!(f, "private-bin {}", bins.join(",")),
            PrivateCache => write!(f, "private-cache"),
            PrivateCwd(path) => write!(f, "private-cwd {}", path),
            PrivateDev => write!(f, "private-dev"),
            PrivateEtc(files) => write!(f, "private-etc {}", files.join(",")),
            PrivateLib(None) => write!(f, "private-lib"),
            PrivateLib(Some(files)) => write!(f, "private-lib {}", files.join(",")),
            PrivateOpt(files) => write!(f, "private-opt {}", files.join(",")),
            PrivateSrv(files) => write!(f, "private-srv {}", files.join(",")),
            PrivateTmp => write!(f, "private-tmp"),
            Protocol(protocols) => write!(f, "protocol {}", join(",", protocols)),
            Quiet => write!(f, "quiet"),
            ReadOnly(path) => write!(f, "read-only {}", path),
            ReadWrite(path) => write!(f, "read-write {}", path),
            Rmenv(name) => write!(f, "rmenv {}", name),
            Seccomp(None) => write!(f, "seccomp"),
            Seccomp(Some(syscalls)) => write!(f, "seccomp {}", syscalls.join(",")),
            SeccompBlockSecondary => write!(f, "seccomp.block-secondary"),
            SeccompDrop(syscalls) => write!(f, "seccomp.drop {}", syscalls.join(",")),
            SeccompErrorAction(action) => write!(f, "seccomp-error-action {}", action),
            ShellNone => write!(f, "shell none"),
            Tmpfs(path) => write!(f, "tmpfs {}", path),
            Tracelog => write!(f, "tracelog"),
            Whitelist(path) => write!(f, "whitelist {}", path),
            WriteableEtc => write!(f, "writable-etc"),
            WritableRunUser => write!(f, "writable-run-user"),
            WritableVar => write!(f, "writable-var"),
            WritableVarLog => write!(f, "writable-var-log"),
            X11None => write!(f, "x11 none"),
        }
    }
}
impl FromStr for Command {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        use Command::*;

        Ok(if line == "allow-debuggers" {
            AllowDebuggers
        } else if line == "allusers" {
            Allusers
        } else if line == "apparmor" {
            Apparmor
        } else if let Some(paths) = line.strip_prefix("bind ") {
            paths
                .split_once(',')
                .map(|(src, dst)| Bind(src.to_string(), dst.to_string()))
                .ok_or(Error::BadBind)?
        } else if let Some(path) = line.strip_prefix("blacklist ") {
            Blacklist(path.to_string())
        } else if let Some(path) = line.strip_prefix("blacklist-nolog ") {
            BlacklistNolog(path.to_string())
        } else if line == "caps" {
            Caps
        } else if line == "caps.drop all" {
            CapsDropAll
        } else if let Some(caps) = line.strip_prefix("caps.drop ") {
            CapsDrop(caps.split(',').map(str::parse).collect::<Result<_, _>>()?)
        } else if let Some(caps) = line.strip_prefix("caps.keep ") {
            CapsKeep(caps.split(',').map(str::parse).collect::<Result<_, _>>()?)
        } else if line == "dbus-user filter" {
            DBusUser(DBusPolicy::Filter)
        } else if line == "dbus-user none" {
            DBusUser(DBusPolicy::None)
        } else if let Some(name) = line.strip_prefix("dbus-user.own ") {
            DBusUserOwn(name.to_string())
        } else if let Some(name) = line.strip_prefix("dbus-user.talk ") {
            DBusUserTalk(name.to_string())
        } else if line == "dbus-system filter" {
            DBusSystem(DBusPolicy::Filter)
        } else if line == "dbus-system none" {
            DBusSystem(DBusPolicy::None)
        } else if let Some(name) = line.strip_prefix("dbus-system.own ") {
            DBusSystemOwn(name.to_string())
        } else if let Some(name) = line.strip_prefix("dbus-system.talk ") {
            DBusSystemTalk(name.to_string())
        } else if line == "disable-mnt" {
            DisableMnt
        } else if let Some(name_and_value) = line.strip_prefix("env ") {
            name_and_value
                .split_once('=')
                .map(|(name, value)| Env(name.to_string(), value.to_string()))
                .ok_or(Error::BadEnv)?
        } else if let Some(hostname) = line.strip_prefix("hostname ") {
            Hostname(hostname.to_string())
        } else if let Some(line) = line.strip_prefix("ignore ") {
            Ignore(line.to_string())
        } else if let Some(other_profile) = line.strip_prefix("include ") {
            Include(other_profile.to_string())
        } else if line == "ipc-namespace" {
            IpcNamespace
        } else if let Some(name) = line.strip_prefix("join-or-start ") {
            JoinOrStart(name.to_string())
        } else if line == "machine-id" {
            MachineId
        } else if line == "memory-deny-write-execute" {
            MemoryDenyWriteExecute
        } else if let Some(path) = line.strip_prefix("mkdir ") {
            Mkdir(path.to_string())
        } else if let Some(path) = line.strip_prefix("mkfile ") {
            Mkfile(path.to_string())
        } else if let Some(sandboxname) = line.strip_prefix("name ") {
            Name(sandboxname.to_string())
        } else if line == "netfilter" {
            Netfilter
        } else if line == "net none" {
            NetNone
        } else if line == "no3d" {
            No3d
        } else if let Some(path) = line.strip_prefix("noblacklist ") {
            Noblacklist(path.to_string())
        } else if line == "nodvd" {
            Nodvd
        } else if let Some(path) = line.strip_prefix("noexec ") {
            Noexec(path.to_string())
        } else if line == "nogroups" {
            Nogroups
        } else if line == "noinput" {
            Noinput
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
        } else if let Some(path) = line.strip_prefix("nowhitelist ") {
            Nowhitelist(path.to_string())
        } else if line == "private" {
            Private(None)
        } else if let Some(path) = line.strip_prefix("private ") {
            Private(Some(path.to_string()))
        } else if let Some(bins) = line.strip_prefix("private-bin ") {
            PrivateBin(bins.split(',').map(String::from).collect())
        } else if line == "private-cache" {
            PrivateCache
        } else if let Some(path) = line.strip_prefix("private-cwd ") {
            PrivateCwd(path.to_string())
        } else if line == "private-dev" {
            PrivateDev
        } else if let Some(files) = line.strip_prefix("private-etc ") {
            PrivateEtc(files.split(',').map(String::from).collect())
        } else if line == "private-lib" {
            PrivateLib(None)
        } else if let Some(libs) = line.strip_prefix("private-lib ") {
            PrivateLib(Some(libs.split(',').map(String::from).collect()))
        } else if let Some(paths) = line.strip_prefix("private-opt ") {
            PrivateOpt(paths.split(',').map(String::from).collect())
        } else if let Some(paths) = line.strip_prefix("private-srv ") {
            PrivateSrv(paths.split(',').map(String::from).collect())
        } else if line == "private-tmp" {
            PrivateTmp
        } else if let Some(protocols) = line.strip_prefix("protocol ") {
            Protocol(
                protocols
                    .split(',')
                    .map(FromStr::from_str)
                    .collect::<Result<_, _>>()?,
            )
        } else if line == "quiet" {
            Quiet
        } else if let Some(path) = line.strip_prefix("read-only ") {
            ReadOnly(path.to_string())
        } else if let Some(path) = line.strip_prefix("read-write ") {
            ReadWrite(path.to_string())
        } else if let Some(name) = line.strip_prefix("rmenv ") {
            Rmenv(name.to_string())
        } else if line == "seccomp" {
            Seccomp(None)
        } else if let Some(syscalls) = line.strip_prefix("seccomp ") {
            Seccomp(Some(syscalls.split(',').map(String::from).collect()))
        } else if line == "seccomp.block-secondary" {
            SeccompBlockSecondary
        } else if let Some(syscalls) = line.strip_prefix("seccomp.drop ") {
            SeccompDrop(syscalls.split(',').map(String::from).collect())
        } else if let Some(action) = line.strip_prefix("seccomp-error-action ") {
            SeccompErrorAction(action.parse()?)
        } else if line == "shell none" {
            ShellNone
        } else if let Some(path) = line.strip_prefix("tmpfs ") {
            Tmpfs(path.to_string())
        } else if line == "tracelog" {
            Tracelog
        } else if let Some(path) = line.strip_prefix("whitelist ") {
            Whitelist(path.to_string())
        } else if line == "writable-etc" {
            WriteableEtc
        } else if line == "writable-run-user" {
            WritableRunUser
        } else if line == "writable-var" {
            WritableVar
        } else if line == "writable-var-log" {
            WritableVarLog
        } else if line == "x11 none" {
            X11None
        } else {
            return Err(Error::BadCommand);
        })
    }
}

//
// Conditional
//

/// A condition with an conditional command
#[non_exhaustive]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Conditional {
    BrowserAllowDrm(Command),
    BrowserDisableU2f(Command),
    HasAppimage(Command),
    HasNet(Command),
    HasNodbus(Command),
    HasNosound(Command),
    HasPrivate(Command),
    HasX11(Command),
}
impl FromStr for Conditional {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut splited_line = line.splitn(2, ' ');
        let con = splited_line.next().unwrap();
        let cmd = splited_line.next().ok_or(Error::EmptyCondition)?;

        if con == "?BROWSER_ALLOW_DRM:" {
            Ok(Self::BrowserAllowDrm(cmd.parse()?))
        } else if con == "?BROWSER_DISABLE_U2F:" {
            Ok(Self::BrowserDisableU2f(cmd.parse()?))
        } else if con == "?HAS_APPIMAGE:" {
            Ok(Self::HasAppimage(cmd.parse()?))
        } else if con == "?HAS_NET:" {
            Ok(Self::HasNet(cmd.parse()?))
        } else if con == "?HAS_NODBUS:" {
            Ok(Self::HasNodbus(cmd.parse()?))
        } else if con == "?HAS_NOSOUND:" {
            Ok(Self::HasNosound(cmd.parse()?))
        } else if con == "?HAS_PRIVATE: " {
            Ok(Self::HasPrivate(cmd.parse()?))
        } else if con == "?HAS_X11:" {
            Ok(Self::HasX11(cmd.parse()?))
        } else {
            Err(Error::BadCondition)
        }
    }
}
impl fmt::Display for Conditional {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BrowserAllowDrm(cmd) => write!(f, "?BROWSER_ALLOW_DRM: {}", cmd),
            Self::BrowserDisableU2f(cmd) => write!(f, "?BROWSER_DISABLE_U2F: {}", cmd),
            Self::HasAppimage(cmd) => write!(f, "?HAS_APPIMAGE: {}", cmd),
            Self::HasNet(cmd) => write!(f, "?HAS_NET: {}", cmd),
            Self::HasNodbus(cmd) => write!(f, "?HAS_NODBUS: {}", cmd),
            Self::HasNosound(cmd) => write!(f, "?HAS_NOSOUND: {}", cmd),
            Self::HasPrivate(cmd) => write!(f, "?HAS_PRIVATE: {}", cmd),
            Self::HasX11(cmd) => write!(f, "?HAS_X11: {}", cmd),
        }
    }
}

macro_rules! values {
    (
        $( #[ $attr:meta ] )*
        pub enum $T:ident {
            $( $variant:ident = $value:literal, )*
            _ = $from_str_error:path,
        }
    ) => {
        $( #[ $attr ] )*
        pub enum $T {
            $( $variant ),*
        }
        impl fmt::Display for $T {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( Self::$variant => write!(f, $value), )*
                }
            }
        }
        impl FromStr for $T {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $( $value => Ok(Self::$variant), )*
                    _ => Err($from_str_error),
                }
            }
        }
    };
}

//
// Capabilities
//

values! {
    /// Caps used by the various `caps` commands
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum Capabilities {
        AuditControl = "audit_control",
        AuditRead = "audit_read",
        AuditWrite = "audit_write",
        BlockSuspend = "block_suspend",
        Bpf = "bpf",
        CheckpointRestore = "checkpoint_restore",
        Chown = "chown",
        DacOverride = "dac_override",
        DacReadSearch = "dac_read_search",
        Fowner = "fowner",
        Fsetid = "fsetid",
        IpcLock = "ipc_lock",
        IpcOwner = "ipc_owner",
        Kill = "kill",
        Lease = "lease",
        LinuxImmutable = "linux_immutable",
        MacAdmin = "mac_admin",
        MacOverride = "mac_override",
        Mknod = "mknod",
        NetAdmin = "net_admin",
        NetBindService = "net_bind_service",
        NetBroadcast = "net_broadcast",
        NetRaw = "net_raw",
        Perfmon = "perfmon",
        Setfcap = "setfcap",
        Setgid = "setgid",
        Setpcap = "setpcap",
        Setuid = "setuid",
        SysAdmin = "sys_admin",
        SysBoot = "sys_boot",
        SysChroot = "sys_chroot",
        SysModule = "sys_module",
        SysNice = "sys_nice",
        SysPacct = "sys_pacct",
        SysPtrace = "sys_ptrace",
        SysRawio = "sys_rawio",
        SysResource = "sys_resource",
        SysTime = "sys_time",
        SysTtyConfig = "sys_tty_config",
        Syslog = "syslog",
        WakeAlarm = "wake_alarm",
        _ = Error::BadCap,
    }
}

//
// DBusPolicy
//

values! {
    /// DBus Policy
    #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
    pub enum DBusPolicy {
        Filter = "filter",
        None = "none",
        _ = Error::BadDBusPolicy,
    }
}

//
// Protocol
//

values! {
    /// A `Protocol` from firejails `protocol` command
    ///
    /// TODO: Support prefixes: `-`, `+` and `=`.
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Protocol {
        Unix = "unix",
        Inet = "inet",
        Inet6 = "inet6",
        Netlink = "netlink",
        Packet = "packet",
        Bluetooth = "bluetooth",
        _ = Error::BadProtocol,
    }
}

macro_rules! seccomp_error_action {
    ( $( $act:ident ),* $(,)? ) => {
        /// A action for firejails `seccomp-error-action`
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
        pub enum SeccompErrorAction {
            Kill,
            Log,
            $( $act ),*
        }
        impl fmt::Display for SeccompErrorAction {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Kill => write!(f, "kill"),
                    Self::Log => write!(f, "log"),
                    $( Self::$act => write!(f, stringify!($act)), )*
                }
            }
        }
        impl FromStr for SeccompErrorAction {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "kill" => Ok(Self::Kill),
                    "log" => Ok(Self::Log),
                    $( stringify!($act) => Ok(Self::$act), )*
                    _ => Err(Error::BadSeccompErrorAction),
                }
            }
        }
    };
}
#[rustfmt::skip]
// Creates `enum SeccompErrorAction`
seccomp_error_action! {
    // gcc -dM -E /usr/include/errno.h | grep -E "^#define E" | cut -d" " -f2 | sort | tr '\n' ',' | sed "s/,/, /g"
    E2BIG, EACCES, EADDRINUSE, EADDRNOTAVAIL, EADV, EAFNOSUPPORT, EAGAIN, EALREADY, EBADE, EBADF,
    EBADFD, EBADMSG, EBADR, EBADRQC, EBADSLT, EBFONT, EBUSY, ECANCELED, ECHILD, ECHRNG, ECOMM,
    ECONNABORTED, ECONNREFUSED, ECONNRESET, EDEADLK, EDEADLOCK, EDESTADDRREQ, EDOM, EDOTDOT, EDQUOT,
    EEXIST, EFAULT, EFBIG, EHOSTDOWN, EHOSTUNREACH, EHWPOISON, EIDRM, EILSEQ, EINPROGRESS, EINTR,
    EINVAL, EIO, EISCONN, EISDIR, EISNAM, EKEYEXPIRED, EKEYREJECTED, EKEYREVOKED, EL2HLT, EL2NSYNC,
    EL3HLT, EL3RST, ELIBACC, ELIBBAD, ELIBEXEC, ELIBMAX, ELIBSCN, ELNRNG, ELOOP, EMEDIUMTYPE,
    EMFILE, EMLINK, EMSGSIZE, EMULTIHOP, ENAMETOOLONG, ENAVAIL, ENETDOWN, ENETRESET, ENETUNREACH,
    ENFILE, ENOANO, ENOBUFS, ENOCSI, ENODATA, ENODEV, ENOENT, ENOEXEC, ENOKEY, ENOLCK, ENOLINK,
    ENOMEDIUM, ENOMEM, ENOMSG, ENONET, ENOPKG, ENOPROTOOPT, ENOSPC, ENOSR, ENOSTR, ENOSYS, ENOTBLK,
    ENOTCONN, ENOTDIR, ENOTEMPTY, ENOTNAM, ENOTRECOVERABLE, ENOTSOCK, ENOTSUP, ENOTTY, ENOTUNIQ,
    ENXIO, EOPNOTSUPP, EOVERFLOW, EOWNERDEAD, EPERM, EPFNOSUPPORT, EPIPE, EPROTO, EPROTONOSUPPORT,
    EPROTOTYPE, ERANGE, EREMCHG, EREMOTE, EREMOTEIO, ERESTART, ERFKILL, EROFS, ESHUTDOWN,
    ESOCKTNOSUPPORT, ESPIPE, ESRCH, ESRMNT, ESTALE, ESTRPIPE, ETIME, ETIMEDOUT, ETOOMANYREFS,
    ETXTBSY, EUCLEAN, EUNATCH, EUSERS, EWOULDBLOCK, EXDEV, EXFULL,
}

//
// Error
//

#[non_exhaustive]
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Invalid bind command")]
    BadBind,
    #[error("Invalid capability")]
    BadCap,
    #[error("Invalid command")]
    BadCommand,
    #[error("Invalid condition")]
    BadCondition,
    #[error("Invalid dbus policy")]
    BadDBusPolicy,
    #[error("Invalid env command")]
    BadEnv,
    #[error("Invalid protocol")]
    BadProtocol,
    #[error("Invalid seccomp-error-action")]
    BadSeccompErrorAction,
    #[error("No command after condition")]
    EmptyCondition,
}
