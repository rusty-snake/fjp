rst2man = find_program('rst2man')
rst2html5 = find_program('rst2html5')

fjp1rst = configure_file(
  input: 'fjp.1.rst.in',
  output: 'fjp.1.rst',
  configuration: {
    'VERSION': meson.project_version(),
  },
)

custom_target(
  'manpage',
  build_by_default: true,
  install: true,
  install_dir: get_option('mandir') / 'man1',
  input: fjp1rst,
  output: 'fjp.1',
  command: [
    rst2man,
    '@INPUT@',
    '@OUTPUT@',
  ],
)

custom_target(
  'manpage-html',
  build_by_default: true,
  install: true,
  install_dir: docdir,
  input: fjp1rst,
  output: 'fjp.1.html5',
  command: [
    rst2html5,
    '@INPUT@',
    '@OUTPUT@',
  ],
)
