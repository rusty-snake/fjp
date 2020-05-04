
 + `*`: needs abstract profile representaion
 + more unit-tests for `utils`
 + Integration teste (possible crates: assert-cmd & assert-fs)
 + improve zsh-completion

edit
----

 - warn/reject `edit abc.local` if no `include abc.local` in `abc.profile`
 - shortcuts (wvc, wc, wruc, wusc, dc, dd, de, di, dp, dx, ap2 ap3, ap, aj, al, ar, ag, ac, as, ax, globals)

cat
---

 - re-add a pager (like git or systemctl)

disable/enable
--------------

 - profile devel option: disable *.inc locals + gloabls.local
 - enable/disable support for /etc/firejail by touching in USER & symln in disabled-dir

grep
----

Like `git grep`, a grep for /etc/firejail/* and ~/.config/firejail/*.

list
----

List all proiles in ~/.config/firejail.

sed
---

Simple mass edit.

check/lint
----------

Check synatx, blacklist, ….
Lints for ordering/sorting, suggest options, check for inconsistents, ….

trash
-----

A trash for user-wide profiles (~/.config/firejail/trash).

 - `--empty`
 - `--list`
 - `--undo=PROFILE`
 - `--delete=PROFILE`

generate-standalone
-------------------

Copy all include profile in a profile.

 - keep `include *.inc` option

build
-----

Reimplement firejail --build

diff*
----

Show the differences between two profiles (e.g. nonewprivs set?, noblacklist ${HOME}/.mozilla?, include allow-python3.inc?)

merge*
-----

Merging two profile.
 Keep noblacklist, ignore, ...
 Remove no*, ... if not set in both.

scan
----

Scan for common mistakes and outdated options in ~/.config/firejail.
