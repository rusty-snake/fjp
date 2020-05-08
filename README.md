firejail-profile
================

![](https://github.com/rusty-snake/fjp/workflows/ShellCheck/badge.svg)
![](https://github.com/rusty-snake/fjp/workflows/Rust/badge.svg)
![rustc 1.41+](https://img.shields.io/badge/rustc-1.41+-blue.svg?logo=rust)
![GPL-3.0-or-later](https://img.shields.io/github/license/rusty-snake/fjp.svg?color=darkred&logo=gnu)
![actively-developed](https://badgen.net/badge/maintenance/actively-developed/00D000)

A commandline program to deal with firejail profiles.

Get started
-----------

1. Install rust: <https://www.rust-lang.org/tools/install>

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ sudo pacman -S rust # Arch Linux
$ sudo dnf install cargo # Fedora
```

2. Clone this repo

```
$ git clone "https://github.com/rusty-snake/fjp.git"
$ cd fjp
```

3. Build and Install

```
$ ./make.sh --prefix=/usr
```

4. Start using it

```
$ fjp --help
```


Examples
--------

Open `~/.config/firejail/firefox.profile` in your editor. If it does not exists yet, it is copied form `/etc/firejail`:

    $ fjp edit firefox

Open `~/.config/firejail/firefox.local` in your editor:

    $ fjp edit firefox.local

Rename `~/.config/firejail` to `~/.config/firejail.disabled` in order to make firejail only using profiles from `/etc/firejail`. And revert it:

    $ fjp disable --user
    $ fjp enable --user

Show firefox and all its includes. Actual firefox.local, globals.local, firefox.profile, firefox-common.local, firefox-common.profile

    $ fjp cat firefox

FAQ
---

**1. What does fjp stand for?**

firejail-profile, but fjp is faster to type.

**2. How can I change the editor?**

fjp reads the `EDITOR` environment-varibale and use `/usr/bin/vim` as fallback.
To use `nano` as editor, just call `EDITOR=nano fjp edit firefox`. In order to make this
persinstent, add `EDITOR=nano` to you `.bashrc`.

**3. How can I change the log level?**

Set the environment-variable `RUST_LOG` to `trace`, `debug`, `info` (default), `warn` or `error`.  
Example: `$ RUST_LOG=debug fjp …`

Changelog
---------

[CHANGELOG.md](CHANGELOG.md)

Contributing
------------

[CONTRIBUTING.md](CONTRIBUTING.md)

License
-------

[GPL-3.0-or-later](COPYING)
