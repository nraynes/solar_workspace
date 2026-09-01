use std::path::Path;

use derive_getters::Getters;
use solar_core::{components::pre_commit::PRE_COMMIT, tools::git::is_git::is_git};

use crate::resources::absolute_git_hooks_path;

mod install;
mod uninstall;

#[derive(Getters)]
pub struct Snapshot {
    is_git: bool,
    pre_commit_file: Option<affirm_fs::File>,
}

impl From<&Path> for Snapshot {
    fn from(value: &Path) -> Self {
        let is_git = is_git(value).unwrap();
        let pre_commit_file = if is_git {
            affirm_fs::File::try_from(absolute_git_hooks_path(value).unwrap().join(PRE_COMMIT))
                .ok()
                .and_then(|x| x.take_and_hold_contents_as_static().ok())
        } else {
            None
        };

        Self {
            is_git,
            pre_commit_file,
        }
    }
}
