#!/bin/bash

set -e

version="$(grep -Eo "[[:digit:]]+\.[[:digit:]]+\.[[:digit:]]+" <<<"$1")"
pre_release="$(grep -Eo -- "-[[:alnum:]]*" <<<"$1" || :)"
pre_release="${pre_release:1}"
build="$(grep -Eo "\+([[:alnum:]]|\.)*" <<<"$1" || :)"
build="${build:1}"

topdir=$(mktemp -dt fjp-build.XXXXXX)
builddir=$(rpm --define "_topdir $topdir" --eval %_builddir)
rpmdir=$(rpm --define "_topdir $topdir" --eval %_rpmdir)
sourcedir=$(rpm --define "_topdir $topdir" --eval %_sourcedir)
specdir=$(rpm --define "_topdir $topdir" --eval %_specdir)
srpmdir=$(rpm --define "_topdir $topdir" --eval %_srcrpmdir)

mkdir -p "$builddir" "$rpmdir" "$sourcedir" "$specdir" "$srpmdir"
cleanup() {
  rm -rf "$topdir"
}
trap cleanup EXIT

sed "s/@VERSION@/$version/g" "$(dirname "$0")"/fjp.spec > "$specdir/fjp.spec"

tar --exclude-vcs-ignore --create --gzip --file "$sourcedir/fjp-$version.tar.gz" .

rpmbuild --nodebuginfo --quiet --define "_topdir $topdir" -bb "$specdir"/fjp.spec

mv "$rpmdir"/*/*.rpm .
