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

set -e

here="$(dirname "$0")"
in_file="$here"/fjp.md
between_file="$here"/fjp.man
out_file="$here"/fjp.1.gz

rm -f fjp.1.gz fjp.man
/usr/bin/pandoc -f gfm -t man -s --strip-comments -o "$between_file" "$in_file" \
 -V date="$(LC_TIME=en_US date)" \
 -V footer="$(head -n3 "$here"/../Cargo.toml | tail -n1 | cut -d" " -f3)" \
 -V section=1 \
 -V title=fjp
gzip -9cfkn "$between_file" > "$out_file"
