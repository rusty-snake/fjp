# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
 - New flags for list: `--incs`, `--locals` and `--profiles`

### Changed
 - Rewrite man-page in reStructuredText
 - MSRV: 1.52.0
 - generate-standalones: rename --keep-inc to --keep-incs (for consistent naming)

## [0.2.0] &ndash; 2021-02-07
### Added
- CI using GitHub Actions
- MSRV: 1.45
- new sub-commands: list, generate-standalone
- new experimental sub-command: diff
- ~~make.sh: more control about the installation paths~~
- ~~make.sh: support striping binaries~~
- ~~make.sh: can build rpms~~
- add short options to disable, enable and edit
- shortnames for profiles on supported sub-commands.
  (e.g. `dc` expands to `disable-common.inc`)
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

## [0.1.0] &ndash; 2020-05-04
### Added
- subcommands: cat, disable, edit, enable, has, rm
- shell completion
- build system make.sh
- manpage (incomplete)


[Unreleased]: https://github.com/rusty-snake/fjp/compare/master...v0.2.0
[0.2.0]: https://github.com/rusty-snake/fjp/releases/tag/v0.2.0
[0.1.0]: https://github.com/rusty-snake/fjp/releases/tag/v0.1.0
