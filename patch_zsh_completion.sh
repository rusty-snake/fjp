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

#TODO: replace this script with rust code in build.rs

in_file="$1"
out_file="$2"
cat >"$out_file" <<EOF
#compdef fjp

autoload -U is-at-least

_disabled_profiles() {
    local disabled_profiles=\$(echo \$HOME/.config/firejail/disabled/*.{inc,local,profile} | sed -E "s;\$HOME/.config/firejail/disabled/;;g")
    if [[ \${#disabled_profiles[@]} -ne 0 ]]; then
        _values 'disabled-profiles' \$disabled_profiles
    fi
}

_system_profiles() {
    _values 'system-profiles' \$(echo /etc/firejail/*.{inc,local,profile} | sed -E "s;/etc/firejail/;;g")
}

_user_profiles() {
    _values 'user-profiles' \$(echo \$HOME/.config/firejail/*.{inc,local,profile} | sed -E "s;\$HOME/.config/firejail/;;g")
}

_all_profiles() {
    _system_profiles
    _user_profiles
}
EOF
tail -n +4 "$in_file" | sed \
  -e "/:PROFILE_NAME -- The name of the profile to show.:/s/_files/_all_profiles/g" \
  -e "/:PROFILE_NAME -- The name of the profile to disable:/s/_files/_user_profiles/g" \
  -e "/:PROFILE_NAME -- The name of the profile to edit.:/s/_files/_all_profiles/g" \
  -e "/:PROFILE_NAME -- The name of the profile to enable:/s/_files/_disabled_profiles/g" \
  -e "/:PROFILE_NAME -- The name of the program to look for a profile.:/s/_files/_all_profiles/g" \
  -e "/:PROFILE_NAMES -- The names of the profiles to delete.:/s/_files/_user_profiles/g" \
>> "$out_file"
