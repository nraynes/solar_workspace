use std::path::Path;

use affirm_fs::Directory;
use derive_getters::Getters;
use solar_core::tools::git::is_git::is_git;

use crate::resources::absolute_git_hooks_path;

mod install;
mod uninstall;

#[derive(Getters)]
pub struct Snapshot {
    is_git: bool,
    hooks_dir: Option<Directory>,
}

impl From<&Path> for Snapshot {
    fn from(value: &Path) -> Self {
        let is_git = is_git(value).unwrap();
        let hooks_dir = if is_git {
            Directory::try_from(absolute_git_hooks_path(value).unwrap())
                .ok()
                .and_then(|x| x.take_and_acquire_contents().ok())
        } else {
            None
        };
        Self { is_git, hooks_dir }
    }
}
