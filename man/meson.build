fjp1rst = configure_file(
  input: 'fjp.1.rst.in',
  output: 'fjp.1.rst',
  configuration: {
    'VERSION': meson.project_version(),
  },
)

make_manpage_sh = find_program(meson.source_root() / 'build-aux/manpage.sh')

custom_target(
  'manpage',
  build_by_default: true,
  install: true,
  install_dir: get_option('mandir'),
  input: fjp1rst,
  output: 'fjp.1.gz',
  command: [
    make_manpage_sh,
    'man',
    '@INPUT@',
    '@OUTPUT@',
  ],
)

custom_target(
  'manpage-html',
  build_by_default: true,
  install: true,
  install_dir: get_option('datadir') / 'doc' / meson.project_name(),
  input: fjp1rst,
  output: 'fjp.1.html5',
  command: [
    make_manpage_sh,
    'html',
    '@INPUT@',
    '@OUTPUT0@',
  ],
)
