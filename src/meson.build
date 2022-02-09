fs = import('fs')

cargo_target_dir = meson.project_build_root() / 'cargo-target-dir'
sources = [
  '../build.rs',
  '../Cargo.lock',
  '../Cargo.toml',
  '../macros/Cargo.toml',
  '../macros/src/lib.rs',
  'cat.rs',
  'cli.rs',
  'diff.rs',
  'disable.rs',
  'edit.rs',
  'enable.rs',
  'generate_standalone.rs',
  'has.rs',
  'list.rs',
  'location.rs',
  'main.rs',
  'profile.rs',
  'profile_stream.rs',
  'rm.rs',
  'utils.rs',
]

cargo = find_program('cargo', fs.expanduser('~/.cargo/bin/cargo'))

cargo_build_cmd = [
  cargo,
  'build',
  '--manifest-path=' + meson.project_source_root() + '/Cargo.toml',
]

if get_option('buildtype') == 'release'
  cargo_build_cmd += ['--release', '--features=color-backtrace']
endif

cargo_build = custom_target(
  'cargo-build',
  console: true,
  build_by_default: true,
  input: sources,
  output: [meson.project_name()],
  env: {
    'FJP_SHELLCOMP_DIR': meson.current_build_dir(),
    'CARGO_TARGET_DIR': cargo_target_dir,
  },
  command: cargo_build_cmd,
)

meson.add_install_script(
  '/usr/bin/env',
    'CARGO_TARGET_DIR=' + cargo_target_dir,
    'MESON_CURRENT_BUILD_DIR=' + meson.current_build_dir(),
    'buildtype=' + get_option('buildtype'),
    'bindir=' + get_option('prefix') / get_option('bindir'),
    'datadir=' + get_option('prefix') / get_option('datadir'),
  '/bin/sh',
    '-e',
    '-u',
    '-c',
    '''
      install -Dm0755 "$CARGO_TARGET_DIR"/"$buildtype"/fjp "$DESTDIR$bindir"/fjp
      install -Dm0644 "$MESON_CURRENT_BUILD_DIR"/fjp.bash "$DESTDIR$datadir"/bash-completion/completions/fjp
      install -Dm0644 "$MESON_CURRENT_BUILD_DIR"/fjp.fish "$DESTDIR$datadir"/fish/completions/fjp.fish
      install -Dm0644 "$MESON_CURRENT_BUILD_DIR"/fjp.zsh "$DESTDIR$datadir"/zsh/site-functions/_fjp
    '''
)
