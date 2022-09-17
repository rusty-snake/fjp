fjp – firejail-profile
======================

[![](https://github.com/rusty-snake/fjp/workflows/Rust%20CI/badge.svg)](https://github.com/rusty-snake/fjp/actions?query=workflow%3A%22Rust+CI%22+event%3Apush+branch%3Amaster)
![MSRV: 1.57](https://img.shields.io/badge/MSRV-1.57-blue.svg?logo=rust)
[![license: GPL-3.0-or-later](https://img.shields.io/static/v1?label=license&message=GPL-3.0-or-later&color=darkred&logo=gnu)](COPYING)
[![maintenance-status: passively-maintained (as of 2022-09-17)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2022--09--17%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)

A handy command line program to work fast and straightforward with firejail profiles.

fjp is a command-line program written in rust, a modern and safe programming language. It allows you to show, edit, compare, disable or remove firejail profiles. And many more features like search, check, sed or merge will come.

Get started
-----------

### Install prebuild binary

```bash
wget -qO- "https://github.com/rusty-snake/fjp/releases/download/v0.3.0/fjp-v0.3.0-x86_64-unknown-linux-musl.tar.xz" | tar -xJf- -C $HOME/.local
```

Read https://rusty-snake.github.io/fjp/#download for more detailed information.

### Build from source

1. Install build dependencies
([rust](https://www.rust-lang.org/tools/install) and
[meson](https://mesonbuild.com/Getting-meson.html))

| Distro | Command(s) |
| ------ | ---------- |
| Arch Linux | `sudo pacman -S rust meson` |
| Debian | `sudo apt install cargo meson` (NOTE: debian stable has likely to old packages) |
| Fedora | `sudo dnf install cargo meson` |
| Other | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup-init.sh`<br>`bash rustup-init.sh --no-modify-path --profile minimal`<br>`pip3 install --user meson` |

[docutils](https://pypi.org/project/docutils/) and [Pygments](https://pypi.org/project/Pygments/) are required too if you want to build the manpage.

2. Clone this repo

```
$ git clone "https://github.com/rusty-snake/fjp.git"
$ cd fjp
```

3. Build and Install

```
$ meson setup --buildtype=release _builddir
$ meson configure _builddir -Dmanpage=true  # Optional
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

#### 1. What does fjp stand for?

firejail-profile, but fjp is faster to type.

#### 2. How can I change the editor?

fjp reads the `EDITOR` environment-varibale and use `/usr/bin/vim` as fallback.
To use `nano` as editor, just call `EDITOR=nano fjp edit firefox`. In order to make this
persistent, add `EDITOR=nano` to your `.bashrc`.

#### 3. How can I change the log level?

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
