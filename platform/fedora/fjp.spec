Name:           fjp
Version:        @VERSION@
Release:        0.dev%{?dist}
Summary:        A commandline program to deal with firejail profiles

License:        GPLv3+
URL:            https://github.com/rusty-snake/fjp
Source0:        fjp-%{version}.tar.gz

BuildRequires:  cargo pandoc

%description
fjp is a command-line program to work more easily with firejail profiles.
It acts like systemctl, but for firejail profiles.


%prep
%autosetup -c


%build
./make.sh configure --prefix=/usr --sysconfdir=/etc --libdir=/usr/lib64
./make.sh build
./make.sh strip


%install
./make.sh install DESTDIR=%{buildroot}


%files
%_bindir/fjp
%_datadir/zsh/site-functions/_fjp
%_datadir/bash-completion/completions/fjp
%_datadir/fish/completions/fjp.fish
%_docdir/fjp/AUTHORS
%_docdir/fjp/CHANGELOG.md
%_docdir/fjp/COPYING
%_docdir/fjp/README.md
%_docdir/fjp/TODO.md
%_mandir/man1/fjp.1.gz
