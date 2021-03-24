project('fjp',
# Remebmer to change version in Cargo.toml and macros/Cargo.toml too.
          version: '0.3.0-dev',
          license: 'GPL-3.0-or-later',
    meson_version: '>= 0.55.0',
  default_options: [ 'strip=true', 'warning_level=3' ]
)

subdir('src')

if get_option('manpage')
  subdir('man')
endif

install_data(
  ['AUTHORS', 'CHANGELOG.md', 'COPYING', 'README.md', 'TODO.md'],
  install_dir: get_option('datadir') / 'doc' / meson.project_name()
)
