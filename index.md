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
from <https://github.com/rusty-snake/fjp/releases>. All you need to do then is
to save the binary in a directory in `$PATH`. In addition you can add one of the
provided shell-completions-script to your preffered shell.

### Install now

#### System

**WARNING:** This command changes the permission of `/usr/local` to be _world writable_!
Don't use it until this is fixed. If you executed this command already, you should
check the permissions of `/usr/local` and change them if necessary.

<details><summary> click me! </summary>
<pre>
wget -qO- "https://github.com/rusty-snake/fjp/releases/download/v0.2.0/fjp-v0.2.0-x86_64-unknown-linux-musl.tar.xz" | sudo tar -xJf- -C /usr/local
</pre>
</details><br>

#### User

```
wget -qO- "https://github.com/rusty-snake/fjp/releases/download/v0.2.0/fjp-v0.2.0-x86_64-unknown-linux-musl.tar.xz" | tar -xJf- -C ~/.local
```

<div style="margin: 1cm;"></div>

Changelog
---------

The current stable release is `0.2.0`.

### Added
- CI using GitHub Actions
- MSRV: 1.45
- new sub-commands: list, generate-standalone
- new experimental sub-command: diff
- add short options to disable, enable and edit
- shortnames for profiles on supported sub-commands.  
  (e.g. `wusc` expands to `whitelist-usr-share-common.inc`)
- edit: If a profile is only found in the system-location ask if it should be copied.

### Changed
- cat: no-globals in now default
- edit \--tmp: consistent behavior: always start editing with the current profile.
  Previously profiles were copied from /etc/firejail if they don't exists in ~/.config/firejail,
  and renamed if the exists (now they will be copied inside ~/.config/firejail).
- has: exit with 100 if no profile could we found.
- Switch from self written make.sh to meson as build-system

### Removed
- edit: `--no-create` has been removed. If you don't want to create the profile,
  just close your editor without saving.
- edit: `--no-copy` has been removed. It is now interactive.

For a full and up-to-date changelog see [CHANGELOG.md](https://github.com/rusty-snake/fjp/blob/master/CHANGELOG.md) on our GitHub repository.
