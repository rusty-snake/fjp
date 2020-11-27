Fjp is a command-line program written in [rust](https://www.rust-lang.org/https://www.rust-lang.org/),
a modern and safe programming language. It allows you to show, edit, compare, disable or remove
firejail profiles. And many more features like search, check, sed or merge will come.

Examples
--------

### Edit

Open `~/.config/firejail/firefox.profile` in your editor. If it does not exists yet, it is copied form `/etc/firejail`:

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

<div style="margin: 1cm;"></div>

### Show (cat)

Show firefox and all its includes. Actual firefox.local, globals.local, firefox.profile, firefox-common.local, firefox-common.profile

    $ fjp cat firefox
    # /home/orion/.config/firejail/firefox.profile:
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

Changelog
---------

The current stable release is `0.1.0`.

#### Added
- subcommands: cat, disable, edit, enable, has, rm
- shell completion
- build system make.sh
- manpage (inclomplete)

For a full and up-to-date changelog see [CHANGELOG.md](https://github.com/rusty-snake/fjp/blob/master/CHANGELOG.md) on our GitHub repo.

Installation
------------

At the moment there are no packages and you need to compile it yourself.  
See [README.md](https://github.com/rusty-snake/fjp/blob/master/README.md#get-started) (dev)
or [README.md](https://github.com/rusty-snake/fjp/tree/v0.1.0#get-started) (stable).
