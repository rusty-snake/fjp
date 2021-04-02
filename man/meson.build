rst2man = find_program('rst2man')
gzip = find_program('gzip')

configure_file(
  input: 'fjp.1.rst.in',
  output: '@BASENAME@',
  configuration: {
    'VERSION': meson.project_version(),
  },
)

custom_target(
  'manpage (rst2man)',
  input: meson.current_build_dir() / 'fjp.1.rst',
  output: 'fjp.1',
  command: [rst2man, '@INPUT@', '@OUTPUT@'],
  build_by_default: true,
)

custom_target(
  'manpage (gzip)',
  input: meson.current_build_dir() / 'fjp.1',
  output: 'fjp.1.gz',
  command: [gzip, '--best', '--keep', '--force', '--no-name', '@INPUT@'],
  build_by_default: true,
  install: true,
  install_dir: get_option('mandir'),
)
