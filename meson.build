project('fjp',
# Remebmer to change version in Cargo.toml and macros/Cargo.toml too.
          version: '0.3.0',
          license: 'GPL-3.0-or-later',
    meson_version: '>= 0.57.0',
  default_options: [ 'strip=true' ],
)

subdir('src')

if get_option('manpage')
  subdir('man')
endif

install_data(
  ['CHANGELOG.md', 'README.md'],
  install_dir: get_option('datadir') / 'doc' / meson.project_name()
)
install_data(
  ['AUTHORS', 'COPYING'],
  install_dir: get_option('datadir') / 'licenses' / meson.project_name()
)
