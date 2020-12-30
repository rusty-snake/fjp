fjp &mdash; firejail-profile
============================

![](https://github.com/rusty-snake/fjp/workflows/Rust/badge.svg)
![](https://img.shields.io/badge/MSRV-1.45-blue.svg?logo=rust)
![](https://img.shields.io/github/license/rusty-snake/fjp.svg?color=darkred&logo=gnu)
![](https://badgen.net/badge/maintenance/actively-developed/00D000)

A handy command line program to work fast and straightforward with firejail profiles.

fjp is a command-line program written in rust, a modern and safe programming language. It allows you to show, edit, compare, disable or remove firejail profiles. And many more features like search, check, sed or merge will come.

Get started
-----------

1. Install build dependencies
([rust](https://www.rust-lang.org/tools/install) and
[meson](https://mesonbuild.com/Getting-meson.html))

| Distro | Command(s) |
| ------ | ---------- |
| Arch Linux | `sudo pacman -S rust meson` |
| Debian | `sudo apt install cargo meson` (NOTE: debian stable has likely to old packages) |
| Fedora | `sudo dnf install cargo meson` |
| Other | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init.sh`<br>`bash rustup-init.sh --no-modify-path --profile minimal`<br>`pip3 install --user meson` |

2. Clone this repo

```
$ git clone "https://github.com/rusty-snake/fjp.git"
$ cd fjp
```

3. Build and Install

```
$ meson setup --buildtype=release _builddir
$ meson compile -C _builddir
$ sudo meson install --no-rebuild -C _builddir
```

4. Start using it

```
$ fjp --help
```


Examples
--------

Open `~/.config/firejail/firefox.profile` in your editor. You will be asked to copy the profile from `/etc/firejail` if it does not exists yet:

    $ fjp edit firefox

Open `~/.config/firejail/firefox.local` in your editor:

    $ fjp edit firefox.local

Rename `~/.config/firejail` to `~/.config/firejail.disabled` in order to make firejail only using profiles from `/etc/firejail`. And revert it:

    $ fjp disable --user
    $ fjp enable --user

Show firefox and all its includes. Actual firefox.local, globals.local, firefox.profile, firefox-common.local, firefox-common.profile

    $ fjp cat firefox

See <https://rusty-snake.github.io/fjp/#examples> for more examples.

FAQ
---

**1. What does fjp stand for?**

firejail-profile, but fjp is faster to type.

**2. How can I change the editor?**

fjp reads the `EDITOR` environment-varibale and use `/usr/bin/vim` as fallback.
To use `nano` as editor, just call `EDITOR=nano fjp edit firefox`. In order to make this
persistent, add `EDITOR=nano` to your `.bashrc`.

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
