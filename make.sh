#!/bin/bash

# Copyright © 2020 rusty-snake
#
# This file is part of fjp
#
# fjp is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# fjp is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

DESTDIR=""
PROFILE="release"
#TODO: FEATURES=()
CARGO_ARGS=()
CONFIG_STATUS_FILE="config.status"
# shellcheck disable=SC2034
VALID_ACTIONS=("_default_" "build" "clean" "configure" "configure-reset" "distclean" "gen-docs" "install" "rpm" "strip" "uninstall")
ACTION="_default_"

unset prefix exec_prefix bindir sbindir libexec sysconfdir libdir datarootdir datadir mandir docdir

# shellcheck disable=SC1090
[ -e "$CONFIG_STATUS_FILE" ] && source "$CONFIG_STATUS_FILE"

version=$(head -n3 Cargo.toml | tail -n1 | cut -d" " -f3)

# Check if an array contains a value. Return: exit-code
# usage: contains NAME_OF_ARRAY VALUE
# NOTE: NAME_OF_ARRAY is the name! and not the array!!
contains() {
  local -n array="$1"
  # NOTE: contained is used as exit-code, therefore 0=true 1=false.
  local contained=1
  for elem in "${array[@]}"; do
    if [ "$2" == "$elem" ]; then
      contained=0
      break
    fi
  done
  return $contained
}

find_cargo() {
  local cargo
  # Using `command -v cargo` here instead of `which cargo` as shellcheck suggest,
  # causes an infinitiy loop, because `command -v` finds the `cargo` function
  # from below if `find_cargo` is started after the declatarion of cargo.
  # which always finds the binary on the disk.
  # shellcheck disable=SC2230
  cargo=$(which cargo 2>/dev/null)
  if [ -z "$cargo" ]; then
    if [ -e "$HOME/.cargo/bin/cargo" ]; then
      cargo="$HOME/.cargo/bin/cargo"
    else
      return 1
    fi
  fi
  echo "$cargo"
}

find_outdir() {
  local out_dir=""
  local invoked_ts_path=""
  local -i newest_ts=0
  local -i current_ts=0
  for invoked_ts_path in target/"$PROFILE"/build/fjp-*/invoked.timestamp; do
    current_ts=$(stat -c%Y "$invoked_ts_path")
    if [[ $newest_ts -lt $current_ts ]]; then
      newest_ts=$current_ts
      out_dir=$(dirname "$invoked_ts_path")
    fi
  done
  echo "$out_dir/out"
}

get_fjp_version() {
  local cargo_toml_versions_line
  cargo_toml_versions_line=$(head -n3 Cargo.toml | tail -n1)
  grep -Eo \
   "[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+(-[[:alnum:]]+)?" \
    <<<"$cargo_toml_versions_line"
}

cargo() {
  if [ ! -v cargo ]; then
    cargo=$(find_cargo || exit 1)
  fi
  $cargo "$@"
}

usage() {
  cat <<END
  USAGE: ./make.sh [OPTIONS] [ACTION]

    OPTIONS:
      --devel                 -- use debug profile (i.e. build w/o optimizations)
      --help  -h  -?          -- show this help
      --prefix=PREFIX         -- [Default: /usr/local]
      --exec_prefix=EPREFIX   -- [Default: PREFIX]
      --bindir=DIR            -- [Default: EPREFIX/bin]
      --sbindir=DIR           -- [Default: EPREFIX/sbin]
      --libexecdir=DIR        -- [Default: EPREFIX/libexec]
      --sysconfdir=DIR        -- [Default: PREFIX/etc]
      --libdir=DIR            -- [Default: EPREFIX/lib]
      --datarootdir=DIR       -- [Default: PREFIX/share]
      --datadir=DIR           -- [Default: DATAROOTDIR]
      --mandir=DIR            -- [Default: DATAROOTDIR/man]
      --docdir=DIR            -- [Default: DATAROOTDIR/doc/fjp
      DESTDIR=<DESTDIR>       -- [Default: "" aka /]

    ACTION:
      The default action is to build and install.

      build      -- build fjp
      clean      -- remove build artifacts
      configure  -- save configuration
      configure-reset -- reset saved configuration
      distclean  -- remove build artifacts and configuration
      gen-docs   -- generate documentation (for the code)
      install    -- install fjp
      rpm        -- build a rpm package (for fedora)
      strip      -- strip all binaries
      uninstall  -- uninstall fjp
END
}

for arg in "$@"; do
  case $arg in
    --help|-h|-\?)
      usage
      exit
    ;;
    --prefix=*)
      prefix="${arg#*=}"
    ;;
    --exec-prefix=*)
      exec_prefix="${arg#*=}"
    ;;
    --bindir=*)
      bindir="${arg#*=}"
    ;;
    --sbindir=*)
      sbindir="${arg#*=}"
    ;;
    --sysconfdir=*)
      sysconfdir="${arg#*=}"
    ;;
    --datadir=*)
      datadir="${arg#*=}"
    ;;
    --libdir=*)
      libdir="${arg#*=}"
    ;;
    --libexecdir=*)
      libexecdir="${arg#*=}"
    ;;
    --mandir=*)
      mandir="${arg#*=}"
    ;;
    --docdir=*)
        docdir="${arg#*=}"
    ;;
    --devel)
      PROFILE="debug"
    ;;
    --*|-?)
      echo "Warning: Unknow commandline argument: $arg"
    ;;
    DESTDIR=*)
      DESTDIR="${arg#*=}"
    ;;
    *)
      if [ "$ACTION" != "_default_" ]; then
        echo "Error: Multiple ACTIONs specified, please use only one."
        exit 1
      fi
      ACTION="$arg"
      if ! contains VALID_ACTIONS "$ACTION"; then
        echo "Error: Invalid ACTION"
        exit 1
      fi
    ;;
  esac
done

[ ! -v prefix ] && prefix="/usr/local"
[ ! -v exec_prefix ] && exec_prefix="$prefix"
[ ! -v bindir ] && bindir="$exec_prefix/bin"
[ ! -v sbindir ] && sbindir="$exec_prefix/sbin"
[ ! -v libexecdir ] && libexecdir="$exec_prefix/libexec"
[ ! -v sysconfdir ] && sysconfdir="$prefix/etc"
[ ! -v libdir ] && libdir="$exec_prefix/lib"
[ ! -v datarootdir ] && datarootdir="$prefix/share"
[ ! -v datadir ] && datadir="$datarootdir"
[ ! -v mandir ] && mandir="$datarootdir/man"
[ ! -v docdir ] && docdir="$datarootdir/doc/fjp"

if [ "$PROFILE" == "release" ]; then
  CARGO_ARGS+=(--release)
fi

CARGO_ARGS+=(--all-features)

case $ACTION in
  _default_)
    ./make.sh "$@" configure
    ./make.sh build
    sudo ./make.sh install
  ;;
  build)
    if [ -e .git ]; then
      FJP_COMMIT=$(git rev-parse --short HEAD --)
      if ! git diff-index --quiet HEAD --; then
        FJP_COMMIT="$FJP_COMMIT-dirty"
      fi
      FJP_VERSION=$(get_fjp_version)
      export FJP_COMMIT FJP_VERSION
    fi
    cargo build "${CARGO_ARGS[@]}"
    ./man/mkman.sh
    OUT_DIR=$(find_outdir)
  ;;
  clean)
    cargo clean
  ;;
  configure)
    cargo=$(find_cargo || exit 1)
    true > "$CONFIG_STATUS_FILE"
    echo "DESTDIR=\"$DESTDIR\"" >> "$CONFIG_STATUS_FILE"
    echo "prefix=\"$prefix\"" >> "$CONFIG_STATUS_FILE"
    echo "exec_prefix=\"$exec_prefix\"" >> "$CONFIG_STATUS_FILE"
    echo "bindir=\"$bindir\"" >> "$CONFIG_STATUS_FILE"
    echo "sbindir=\"$sbindir\"" >> "$CONFIG_STATUS_FILE"
    echo "libexecdir=\"$libexecdir\"" >> "$CONFIG_STATUS_FILE"
    echo "sysconfdir=\"$sysconfdir\"" >> "$CONFIG_STATUS_FILE"
    echo "libdir=\"$libdir\"" >> "$CONFIG_STATUS_FILE"
    echo "datarootdir=\"$datarootdir\"" >> "$CONFIG_STATUS_FILE"
    echo "datadir=\"$datadir\"" >> "$CONFIG_STATUS_FILE"
    echo "mandir=\"$mandir\"" >> "$CONFIG_STATUS_FILE"
    echo "docdir=\"$docdir\"" >> "$CONFIG_STATUS_FILE"
    echo "PROFILE=\"$PROFILE\"" >> "$CONFIG_STATUS_FILE"
    echo "cargo=\"$cargo\"" >> "$CONFIG_STATUS_FILE"
  ;;
  configure-reset)
    rm "$CONFIG_STATUS_FILE"
  ;;
  distclean)
    rm -rf target "$CONFIG_STATUS_FILE"
  ;;
  gen-docs)
    #TODO: CARGO_ARGS+="--no-deps"
    cargo doc "${CARGO_ARGS[@]}"
    cargo doc "${CARGO_ARGS[@]}" --package macros
    echo "You can now read the docs by opening one of the urls below in your browser."
    echo "  file://$PWD/target/doc/fjp/index.html"
    echo "  file://$PWD/target/doc/macros/index.html"
  ;;
  install)
    OUT_DIR=$(find_outdir)
    install -Dm0755 target/release/fjp "$DESTDIR$bindir"/fjp
    install -Dm0644 "$OUT_DIR"/_fjp "$DESTDIR$datadir"/zsh/site-functions/_fjp
    install -Dm0644 "$OUT_DIR"/fjp.bash "$DESTDIR$datadir"/bash-completion/completions/fjp
    install -Dm0644 "$OUT_DIR"/fjp.fish "$DESTDIR$datadir"/fish/completions/fjp.fish
    install -Dm0644 AUTHORS "$DESTDIR$docdir"/AUTHORS
    install -Dm0644 CHANGELOG.md "$DESTDIR$docdir"/CHANGELOG.md
    install -Dm0644 COPYING "$DESTDIR$docdir"/COPYING
    install -Dm0644 README.md "$DESTDIR$docdir"/README.md
    install -Dm0644 TODO.md "$DESTDIR$docdir"/TODO.md
    install -Dm0644 man/fjp.1.gz "$DESTDIR$mandir"/man1/fjp.1.gz
  ;;
  rpm)
    ./platform/fedora/mkrpm.sh "$version"
  ;;
  strip)
    strip target/release/fjp
  ;;
  uninstall)
    rm -rf \
      "$DESTDIR$bindir"/fjp \
      "$DESTDIR$docdir" \
      "$DESTDIR$datadir"/bash-completion/completions/fjp \
      "$DESTDIR$datadir"/fish/completions/fjp.fish \
      "$DESTDIR$datadir"/zsh/site-functions/_fjp \
      "$DESTDIR$mandir"/man1/fjp.1.gz
  ;;
esac

# vim: set ts=2 sw=2 expandtab:
