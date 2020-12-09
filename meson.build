project('fjp',
# Remebmer to change version in Cargo.toml, macros/Cargo.toml and fjp.spec.
          version: '0.2.0-dev',
          license: 'GPL-3.0-or-later',
    meson_version: '>= 0.49.0',
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
