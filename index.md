Fjp is a command-line program written in [Rust](https://www.rust-lang.org/),
a modern and safe programming language. It allows you to show, edit, compare, disable or remove
[firejail](https://firejail.wordpress.com/) profiles. And many more features like search, check,
sed or merge will come.

Examples
--------

### Edit

Open `~/.config/firejail/firefox.profile` in your editor. If it does not exists yet,
you will be asked whether it should be copied from `/etc/firejail`:

    $ fjp edit firefox

Open `~/.config/firejail/firefox.local` in your editor:

    $ fjp edit firefox.local

Open `~/.config/firejail/firefox.local` in nano:

    $ EDITOR=nano fjp edit firefox.local

Open `~/.config/firejail/supertuxkart.profile` in your editor and discard all change on exit. `supertuxkart.profile` is copied from `/etc/firejail` if necessary.

    $ fjp edit --tmp supertuxkart.profile

<div style="margin: 1cm;"></div>

### Enable/Disable

Rename `~/.config/firejail` to `~/.config/firejail.disabled` in order to make firejail only using profiles from `/etc/firejail`. And revert it:

    $ fjp disable --user
    $ fjp enable --user

Move `~/.config/firejail/disable-common.local` to `~/.config/firejail/disabled/disable-common.local` where it isn't read by firejail. And revert it:

    $ fjp disable disable-common.local
    $ fjp enable disable-common.local

<div style="margin: 1cm;"></div>

### Show (cat)

Show firefox and all its includes. Actual firefox.local, globals.local, firefox.profile, firefox-common.local, firefox-common.profile

    $ fjp cat firefox
    # /etc/firejail/firefox.profile:
    # Firejail profile for firefox
    # Description: Safe and easy web browser from Mozilla
    # This file is overwritten after every install/update
    # Persistent local customizations
    include firefox.local
    # Persistent global definitions
    include pre-globals.local
    
    noblacklist ${HOME}/.cache/mozilla
    noblacklist ${HOME}/.mozilla
    
    mkdir ${HOME}/.cache/mozilla/firefox
    mkdir ${HOME}/.mozilla
    whitelist ${HOME}/.cache/mozilla/firefox
    whitelist ${HOME}/.mozilla
    . . .

<div style="margin: 1cm;"></div>

### Remove

Delete `~/.config/firejail/google-chrome.profile`:

    $ fjp rm google-chrome

<div style="margin: 1cm;"></div>

### List

List all files in `~/.config/firejail`.

    $ fjp list
    brave.local
    celluloid.local
    chromium-common.local
    dconf-editor.local
    default.local
    epiphany.profile
    firefox-common.local
    firefox.local
    firejailed-tor-browser.profile
    keepassxc.local
    totem.local
    vivaldi.local
    youtube-dl.local

<div style="margin: 1cm;"></div>

Download
--------

You can download a tar.xz archive with a statically linked 64-bit musl binary
from <https://github.com/rusty-snake/fjp/releases/tag/v{{ site.fjp_version }}>.
All you need to do then is to save the binary in a directory in `$PATH`. In
addition you can add one of the provided shell-completions-script to your
preffered shell.

To make the installation easier you can execute one of the commands below to
install fjp.

**System**

```
wget -qO- "https://github.com/rusty-snake/fjp/releases/download/v{{ site.fjp_version }}/fjp-v{{ site.fjp_version }}-x86_64-unknown-linux-musl.tar.xz" | sudo tar -xJf- -C /usr/local
```

**User**

```
wget -qO- "https://github.com/rusty-snake/fjp/releases/download/v{{ site.fjp_version }}/fjp-v{{ site.fjp_version }}-x86_64-unknown-linux-musl.tar.xz" | tar -xJf- -C ~/.local
```

{% if site.fjp_rc_version %}

### Download the latest release candidate

Alternatively you can install and test the latest release candidate from
<https://github.com/rusty-snake/fjp/releases/tag/v{{ site.fjp_rc_version }}>.

**System**

```
wget -qO- "https://github.com/rusty-snake/fjp/releases/download/v{{ site.fjp_rc_version }}/fjp-v{{ site.fjp_rc_version }}-x86_64-unknown-linux-musl.tar.xz" | sudo tar -xJf- -C /usr/local
```

**User**

```
wget -qO- "https://github.com/rusty-snake/fjp/releases/download/v{{ site.fjp_rc_version }}/fjp-v{{ site.fjp_rc_version }}-x86_64-unknown-linux-musl.tar.xz" | tar -xJf- -C ~/.local
```
{% endif %}

<div style="margin: 1cm;"></div>

Changelog
---------

The current stable release is `{{ site.fjp_version }}`.

### Added
 - New flags for list: `--incs`, `--locals` and `--profiles`

### Changed
 - Rewrite man-page in reStructuredText
 - MSRV: 1.52.0
 - generate-standalones: rename --keep-inc to --keep-incs (for consistent naming)
 - updated shortnames

For a full and up-to-date changelog see [CHANGELOG.md](https://github.com/rusty-snake/fjp/blob/master/CHANGELOG.md) on our GitHub repository.
