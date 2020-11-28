app_bin = meson.project_name()

sources = [
  '../build.rs',
  '../Cargo.lock',
  '../Cargo.toml',
  'cat.rs',
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

outputs = [
  app_bin,
  app_bin + '.bash',
  app_bin + '.fish',
  app_bin + '.zsh',
]

cargo_script = find_program(join_paths(meson.source_root(), 'build-aux/cargo.sh'))
custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: outputs,
  console: true,
  command: [
    cargo_script,
    meson.build_root(),
    meson.source_root(),
    meson.current_build_dir(),
    get_option('buildtype'),
    app_bin,
  ]
)

install_script = find_program(join_paths(meson.source_root(), 'build-aux/install.sh'))
meson.add_install_script(
  install_script,
  meson.current_build_dir(),
  app_bin,
  get_option('bindir'),
  get_option('datadir'),
)
