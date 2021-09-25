#!/bin/bash

# Copyright © 2020,2021 The fjp Authors
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

set -euo pipefail

# cd into the project directory
cd -P -- "$(readlink -e "$(dirname "$0")")"

me="$(basename "$0")"

#TODO: exec > >(tee "$me.log")

# Do not run if an old outdir exists
[ -d outdir ] && { echo "$me: Please delete 'outdir' first."; exit 1; }

# Check presents of non-standard programs (everything except coreutils and built-ins)
if ! command -v cargo >&-; then
	echo "$me: Missing requirement: cargo is not installed or could not be found."
	echo "Please make sure cargo is installed and in \$PATH."
	exit 1
fi
if ! command -v git >&-; then
	echo "$me: Missing requirement: git is not installed or could not be found."
	echo "Please make sure git is installed and in \$PATH."
	exit 1
fi
if ! command -v podman >&-; then
	echo "$me: Missing requirement: podman is not installed or could not be found."
	echo "Please make sure podman is installed and in \$PATH."
	exit 1
fi
if ! command -v xz >&-; then
	echo "$me: Missing requirement: xz is not installed or could not be found."
	echo "Please make sure xz is installed and in \$PATH."
	exit 1
fi

# Pull alpine image if necessary
if [[ -z "$(podman image list --noheading alpine:latest)" ]]; then
	podman pull docker.io/library/alpine:latest
fi

# Check if we are allowed to run podman
if [[ "$(podman run --rm alpine:latest echo "hello")" != "hello" ]]; then
	echo "$me: podman does not seem to work correctly."
	exit 1
fi

IFS='#' read -r PROJECT VERSION < <(basename "$(cargo pkgid)")
VERSION="v$VERSION"

# Vendor all dependencies
cargo --color=never --locked vendor vendor
[ -d .cargo ] && mv -v .cargo .cargo.bak
mkdir -v .cargo
trap "rm -rv .cargo && [ -d .cargo.bak ] && mv -v .cargo.bak .cargo" EXIT
echo "$me: Creating .cargo/config.toml"
cat > .cargo/config.toml <<EOF
[source.crates-io]
replace-with = "vendored-sources"
[source.vendored-sources]
directory = "vendor"
EOF

mkdir -v outdir

# Create the source archive
echo "$me: Start to pack the source archive"
git archive --format=tar --prefix="$PROJECT-$VERSION/" -o "outdir/$PROJECT-$VERSION.src.tar" "$VERSION"
tar --xform="s,^,$PROJECT-$VERSION/," -rf "outdir/$PROJECT-$VERSION.src.tar" .cargo vendor
xz "outdir/$PROJECT-$VERSION.src.tar"

# Build the project
echo "$me: Starting build"
SOURCEDIR="/sourcedir"
BUILDDIR="/builddir"
INSTALLDIR="/installdir"
podman run --rm --security-opt=no-new-privileges --cap-drop=all \
	-v ./outdir:/outdir:z --tmpfs "$SOURCEDIR" --tmpfs "$BUILDDIR" \
	--tmpfs "$INSTALLDIR:mode=0755" -w "$SOURCEDIR" alpine:latest sh -euo pipefail -c "
		apk update
		apk upgrade ||:
		apk add bash curl gcc musl-dev ninja py3-docutils py3-pip py3-pygments xz ||:
		pip3 install meson
		curl --proto '=https' --tlsv1.3 -sSf 'https://sh.rustup.rs' | sh -s -- -y --profile minimal
		source ~/.cargo/env
		tar --strip=1 -xf '/outdir/$PROJECT-$VERSION.src.tar.xz'
		export CARGO_FROZEN=true
		meson setup --buildtype=release --prefix=/ -Dmanpage=true '$BUILDDIR' '$SOURCEDIR'
		meson compile -C '$BUILDDIR'
		meson install --no-rebuild --destdir='$INSTALLDIR' -C '$BUILDDIR'
		strip '$INSTALLDIR/bin/fjp'
		tar -cJf '/outdir/$PROJECT-$VERSION-x86_64-unknown-linux-musl.tar.xz' -C '$INSTALLDIR' .
	"

# Compute checksums
(cd outdir; sha256sum *.tar.xz) > outdir/SHA256SUMS
(cd outdir; sha512sum *.tar.xz) > outdir/SHA512SUMS

if [[ -n "${MINISIGN:-}" ]] && command -v minisign >&-; then
	minisign -S -s "$MINISIGN" -m outdir/*
fi

echo "Success!"
