# fjp

fjp - work with firejail profiles

## USAGE

`fjp <SUBCOMMAND> [<SUBCOMMAND-FLAGS>] <PROFILE_NAME>`

## DESCRIPTION

A commandline program to deal with firejail profiles.

## SUBCOMMANDS

### cat

**--no-globals**  

**--no-locals**  

**--no-pager**  

**--no-redirects**  

### disable

**--list**  
List all disabled profiles.

**--user**

### edit

**--no-copy**  
Do not copy the profile if it does not exists.

**--no-create**  

### enable

**--user**

### has

### help

### list

List all files in ~/.config/firejail.

### rm

Delete a profile in the user location.

## ENVIRONMENT

$EDITOR - The editor used by fjp edit. If it is not set, /usr/bin/vim is used as fallback.

## SEE ALSO

firejail(1), firejail-profile(5)
