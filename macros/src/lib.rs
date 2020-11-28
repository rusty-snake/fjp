extern crate proc_macro;

use std::env;
use std::process;

/// Create a [`String`](std::string::String) containing the version of fjp.
///
/// ### Format:
///  - `VERSION[+COMMIT[-dirty]]`
///  - VERSION: `0.2.0` or `0.2.0-dev`
///  - COMMIT: `0616df2` if VERSION has a pre-release identifier
///    and `FJP_COMMIT` (env-var) is set or `.git` (dir) exists.
///  - `-dirty`: if a dirty state could be determined
#[proc_macro]
pub fn fjp_version(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use env::var;
    use process::Command;

    fn git_commit() -> Option<String> {
        Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .into_owned()
                    .trim()
                    .to_string()
            })
    }

    fn is_dirty() -> bool {
        Command::new("git")
            .args(&["diff-index", "--quiet", "HEAD", "--"])
            .status()
            .ok()
            .and_then(|st_code| st_code.code())
            .map_or(false, |code| code == 1)
    }

    let mut version = env!("CARGO_PKG_VERSION").to_string();

    if !env!("CARGO_PKG_VERSION_PRE").is_empty() {
        let commit = var("FJP_COMMIT").ok().or_else(git_commit);
        if let Some(commit) = commit {
            version.push('+');
            version.push_str(&commit);

            if is_dirty() {
                version.push_str("-dirty");
            }
        }
    }

    proc_macro::TokenStream::from(proc_macro::TokenTree::from(proc_macro::Literal::string(
        &version,
    )))
}
