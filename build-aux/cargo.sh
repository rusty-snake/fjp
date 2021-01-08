#!/bin/bash

set -e

MESON_BUILD_ROOT="$1"
MESON_SOURCE_ROOT="$2"
MESON_CURRENT_BUILD_DIR="$3"
BUILDTYPE="$4"
APP_BIN="$5"

export CARGO_TARGET_DIR="$MESON_BUILD_ROOT"/target
#export CARGO_HOME="$CARGO_TARGET_DIR"/cargo-home
export FJP_SHELLCOMP_DIR="$MESON_CURRENT_BUILD_DIR"

export PATH="${PATH}:${HOME}/.cargo/bin"

if [[ $BUILDTYPE == "release" ]]; then
    echo "RELEASE MODE"
    cargo build --manifest-path "$MESON_SOURCE_ROOT"/Cargo.toml --release --features=color-backtrace
    cp "$CARGO_TARGET_DIR"/release/"$APP_BIN" "$MESON_CURRENT_BUILD_DIR"
else
    echo "DEBUG MODE"
    cargo build --manifest-path "$MESON_SOURCE_ROOT"/Cargo.toml --verbose
    cp "$CARGO_TARGET_DIR"/debug/"$APP_BIN" "$MESON_CURRENT_BUILD_DIR"
fi
