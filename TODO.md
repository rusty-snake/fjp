
 + more unit-tests for `utils`
 + Integration tests (possible crates: assert-cmd & assert-fs)
 + improve zsh-completion
 + user editable shortcuts
 + rethink the syntax for profile::Profile::complete_name
 + Rewrite the manpage using roff (https://www.cyberciti.biz/faq/linux-unix-creating-a-manpage/)

edit
----

 - warn/reject `edit abc.local` if no `include abc.local` in `abc.profile`
 - shortcuts (globals, pre-globals, post-globals)

cat
---

disable/enable
--------------

 - profile devel option: disable *.inc locals + gloabls.local
 - enable/disable support for /etc/firejail by touching in USER & symln in disabled-dir

grep
----

Like `git grep`, a grep for /etc/firejail/* and ~/.config/firejail/*.


sed
---

Simple mass edit.

check/lint
----------

Check synatx, blacklist, ….
Lints for ordering/sorting, suggest options, check for inconsistents, ….
check-blacklist

fix
---

fix some auto-fixable lints

trash
-----

A trash for user-wide profiles (~/.config/firejail/trash).

 - `--empty`
 - `--list`
 - `--undo=PROFILE`
 - `--delete=PROFILE`

mv
--

rename profiles

cp
--

copy profiles

generate-standalone
-------------------

 - `-o OUTPUT_FILE`
 - `--keep-locals`
 - Add notes e.g.
```
#
# Beginn firefox.local
#
blacklist ${HOME}/Downloads/NotForFirefox
#
# End firefox.local
#
…
```

build
-----

Reimplement firejail --build

diff
----

 - Handel `include *.{local,profile}`
 - implement some config-file support to set a default format for it
 - show files side-by-side with `--format=color`
 - format=color: `private-etc foo,bar`, `private-etc foo` should only highligt `,bar`

merge
-----

Merging two profile.
 Keep noblacklist, ignore, ...
 Remove no*, ... if not set in both.

scan
----

Scan for common mistakes and outdated options in ~/.config/firejail.

query
-----

Some kind of higherlevel grep (e.g. exist a blacklist for foo/bar; list all redirect profiles; ...)

gui
---

A gui for all of this.
