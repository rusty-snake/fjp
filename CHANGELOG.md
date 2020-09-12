# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- CI using GitHub Actions
- MSRV: 1.42
- new subcommands: list, diff, generate-standalone
- make.sh: more controll about the installation paths
- make.sh: support striping binaries
- make.sh: can build rpms
- add short options

### Changed
- cat: no-globals in now default
- edit --tmp: consistent behavior: always start editing with the current profile.
  Previously profiles were copied from /etc/firejail if they don't exists in ~/.config/firejail,
  and renamed if the exists (now they will be copied inside ~/.config/firejail).
- has: exit with 100 if no profile could we found

## [0.1.0] - 2020-05-04
### Added
- subcommands: cat, disable, edit, enable, has, rm
- shell completion
- build system make.sh
- manpage (inclomplete)


[Unreleased]: https://github.com/rusty-snake/fjp/compare/master...v0.1.0
[0.1.0]: https://github.com/rusty-snake/fjp/releases/tag/v0.1.0
