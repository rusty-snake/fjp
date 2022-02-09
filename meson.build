project('fjp',
# Remebmer to change version in Cargo.toml and macros/Cargo.toml too.
          version: '0.4.0-dev',
          license: 'GPL-3.0-or-later',
    meson_version: '>= 0.57.0',
  default_options: [ 'strip=true' ],
)

docdir = get_option('datadir') / 'doc' / meson.project_name()

subdir('src')

if get_option('manpage')
  subdir('man')
endif

install_data(
  'CHANGELOG.md', 'README.md', 'COPYING', 'AUTHORS',
  install_dir: docdir
)
