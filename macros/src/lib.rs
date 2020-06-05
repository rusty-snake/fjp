extern crate proc_macro;

use quote::quote;
use std::env;
use std::ffi;
use std::fs;
use std::process;

#[proc_macro]
/// Create a `static FJP_VERSION: &str`
///
/// fjp_version tries very hard to get a version and commit, if no version can be found
/// it will use `"Unknow"`.
///
/// ## Version
///
/// 1. Check env-var `FJP_VERSION`
/// 2. Fallback to `Cargo.toml`
///
/// ## Commit
///
/// 1. Check env-var `FJP_COMMIT`
/// 2. Fallback to `git`
pub fn fjp_version(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use env::var;
    use ffi::OsStr;
    use fs::{read_dir, read_to_string};
    use process::Command;

    let pwd_content: Option<Vec<ffi::OsString>> = read_dir(".").ok().map(|dir_entrys| {
        dir_entrys
            .filter_map(|dir_entry| {
                if let Ok(dir_entry) = dir_entry {
                    Some(dir_entry.file_name())
                } else {
                    None
                }
            })
            .collect()
    });

    let mut version: Option<String> = var("FJP_VERSION").ok();
    if version.is_none() {
        if let Some(ref files) = pwd_content {
            if files.iter().any(|name| name == OsStr::new("Cargo.toml")) {
                if let Ok(data) = read_to_string("Cargo.toml") {
                    let version_line = data.lines().nth(2).unwrap();
                    version = Some(version_line[11..version_line.len() - 1].to_string())
                }
            }
        }
    }

    let mut commit: Option<String> = var("FJP_COMMIT").ok();
    if commit.is_none() {
        if let Some(ref files) = pwd_content {
            if files.iter().any(|name| name == ".git") {
                let git_commit_hash: Option<String> = Command::new("git")
                    .args(&["rev-parse", "--short", "HEAD", "--"])
                    .output()
                    .ok()
                    .map(|output| {
                        String::from_utf8_lossy(&output.stdout)
                            .into_owned()
                            .trim()
                            .to_string()
                    });
                let has_dirty_worktree: Option<bool> = Command::new("git")
                    .args(&["diff-index", "--quiet", "HEAD", "--"])
                    .status()
                    .ok()
                    .and_then(|st_code| st_code.code())
                    .and_then(|code| match code {
                        0 => Some(false),
                        1 => Some(true),
                        _ => None,
                    });
                if let Some(hash) = git_commit_hash {
                    if let Some(is_dirty) = has_dirty_worktree {
                        if is_dirty {
                            commit = Some(hash + "-dirty")
                        } else {
                            commit = Some(hash)
                        }
                    } else {
                        commit = Some(hash)
                    }
                }
            }
        }
    }

    let full_version: String;
    if let Some(version) = version {
        if let Some(commit) = commit {
            full_version = version + "+" + &commit
        } else {
            full_version = version
        }
    } else {
        full_version = "Unknow".to_string()
    };

    let tokens: proc_macro2::TokenStream = quote! {
        static FJP_VERSION: &str = #full_version;
    };

    tokens.into()
}
