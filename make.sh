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

DESTDIR="/usr/local"
PROFILE="release"
#TODO: FEATURES=()
CARGO_ARGS=()
CONFIG_STATUS_FILE="config.status"
# shellcheck disable=SC2034
VALID_ACTIONS=("_default_" "build" "clean" "configure" "configure-reset" "distclean" "install" "uninstall")
ACTION="_default_"

# shellcheck disable=SC1090
[ -e "$CONFIG_STATUS_FILE" ] && source "$CONFIG_STATUS_FILE"

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

usage() {
  cat <<END
  USAGE: ./make.sh [OPTIONS] [ACTION]

    OPTIONS:
      --devel       -- use debug profile (i.e. build w/o optimizations)
      --help        -- show this help
      --prefix=PATH -- install files under PATH instead of /usr/local

    ACTION:
      The default action is to build and install.

      build      -- build fjp
      clean      -- remove build artifacts
      configure  -- save configuration
      configure-reset -- reset saved configuration
      distclean  -- remove build artifacts and configuration
      install    -- install fjp
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
      DESTDIR="${arg#*=}"
    ;;
    --devel)
      PROFILE="debug"
    ;;
    --*|-?)
      echo "Warning: Unknow commandline argument: $arg"
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
    COMMIT=$(git describe | cut -s -d- -f3)
    export COMMIT
    cargo build "${CARGO_ARGS[@]}"
    ./man/mkman.sh
    OUT_DIR=$(find_outdir)
    ./patch_zsh_completion.sh "$OUT_DIR"/_fjp "$OUT_DIR"/_fjp.patched
  ;;
  clean)
    cargo clean
  ;;
  configure)
    true > "$CONFIG_STATUS_FILE"
    echo "DESTDIR=\"$DESTDIR\"" >> "$CONFIG_STATUS_FILE"
    echo "PROFILE=\"$PROFILE\"" >> "$CONFIG_STATUS_FILE"
  ;;
  configure-reset)
    rm "$CONFIG_STATUS_FILE"
  ;;
  distclean)
    rm -rf target "$CONFIG_STATUS_FILE"
  ;;&
  install)
    OUT_DIR=$(find_outdir)
    install -Dm0755 target/release/fjp "$DESTDIR"/bin/fjp
    install -Dm0644 "$OUT_DIR"/_fjp.patched "$DESTDIR"/share/zsh/site-functions/_fjp
    install -Dm0644 "$OUT_DIR"/fjp.bash "$DESTDIR"/share/bash-completion/completions/fjp
    install -Dm0644 "$OUT_DIR"/fjp.fish "$DESTDIR"/share/fish/completions/fjp.fish
    install -Dm0644 AUTHORS "$DESTDIR"/share/doc/fjp/AUTHORS
    install -Dm0644 CHANGELOG.md "$DESTDIR"/share/doc/fjp/CHANGELOG.md
    install -Dm0644 COPYING "$DESTDIR"/share/doc/fjp/COPYING
    install -Dm0644 README.md "$DESTDIR"/share/doc/fjp/README.md
    install -Dm0644 TODO.md "$DESTDIR"/share/doc/fjp/TODO.md
    install -Dm0644 man/fjp.1.gz "$DESTDIR"/share/man/man1/fjp.1.gz
  ;;
  uninstall)
    rm -rf \
      "$DESTDIR"/bin/fjp \
      "$DESTDIR"/share/doc/fjp \
      "$DESTDIR"/share/bash-completion/completions/fjp \
      "$DESTDIR"/share/fish/completions/fjp.fish \
      "$DESTDIR"/share/zsh/site-functions/_fjp \
      "$DESTDIR"/share/man/man1/fjp.1.gz
  ;;
esac

