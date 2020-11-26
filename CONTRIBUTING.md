# How to contribute to firejail-profile

First of all, thanks for contibuting.

## Issues

You can and should open an issue if you …

- … have an idea for new features.
- … have a question.
- … found a bug.

You should do a quick search to avoid duplicated issues.

### Bugs

Please include the following informations:

- Distripution (e.g. Arch Linux)
- firejail-profile version (`fjp --version`)
- rust version (`rustc --version`)

## Pull Requests

If you contibute code, please use rustfmt to format it.

New documentation comments must use rustdocs [intra-doc] feature.
[intra-doc]: https://doc.rust-lang.org/rustdoc/linking-to-items-by-name.html

### Dependency Specification Rules

- Omit patch versions.
- If features are specified, the [dependencies.foo] syntax is used.
- Features dependencies are ordered alphabetically with following Ordering.
  - Ordering:
    - [dependencies];
    - [dependencies.foo];
    - [dependencies.bar]
      -- optional=true;
    - path;
    - build;
    - dev;

### use Structs,functions,Traits etc in function body.

- macros/src/lib.rs
- src/utils.rs
