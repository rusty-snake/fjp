project('fjp',
# Remebmer to change version in Cargo.toml and macros/Cargo.toml too.
          version: '0.2.0-rc2',
          license: 'GPL-3.0-or-later',
    meson_version: '>= 0.55.0',
  default_options: [
    'warning_level=3',
    'strip=true',
  ]
)

subdir('src')

install_data(
  ['AUTHORS', 'CHANGELOG.md', 'COPYING', 'README.md', 'TODO.md'],
  install_dir: get_option('datadir') / 'doc/fjp'
)
