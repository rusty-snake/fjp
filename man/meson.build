rst2man = find_program('rst2man')
gzip = find_program('gzip')

custom_target(
  'manpage (rst2man)',
  output: 'fjp.1',
  input: 'fjp.1.rst',
  command: [rst2man, '@INPUT@', '@OUTPUT@'],
  build_by_default: true,
)

custom_target(
  'manpage (gzip)',
  output: 'fjp.1.gz',
  input: meson.current_build_dir() / 'fjp.1',
  command: [gzip, '--best', '--keep', '--no-name', '@INPUT@'],
  build_by_default: true,
  install: true,
  install_dir: get_option('mandir'),
)

# Preview the manpage:
#  meson compile -C _builddir man/preview
run_target(
  'preview',
  command: ['sh', '-c', 'rst2man "$MESON_SOURCE_ROOT/$MESON_SUBDIR/fjp.1.rst" | man -l -'],
)
