#!/bin/bash

set -e

MESON_CURRENT_BUILD_DIR="$1"
APP_BIN="$2"
BINDIR="$MESON_INSTALL_DESTDIR_PREFIX/$3"
DATADIR="$MESON_INSTALL_DESTDIR_PREFIX/$4"

install -Dm0755 "$MESON_CURRENT_BUILD_DIR/$APP_BIN" "$BINDIR"/"$APP_BIN"
install -Dm0644 "$MESON_CURRENT_BUILD_DIR"/fjp.bash "$DATADIR"/bash-completion/completions/fjp
install -Dm0644 "$MESON_CURRENT_BUILD_DIR"/fjp.fish "$DATADIR"/fish/completions/fjp.fish
install -Dm0644 "$MESON_CURRENT_BUILD_DIR"/fjp.zsh "$DATADIR"/zsh/site-functions/_fjp
