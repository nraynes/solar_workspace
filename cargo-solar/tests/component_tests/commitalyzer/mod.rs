mod install;
mod uninstall;
mod upgrade;

use std::path::Path;

use derive_getters::Getters;
use solar_core::{
    components::commitalyzer::{COMMIT_MSG_NAME, COMMIT_RULES_NAME},
    tools::git::is_git::is_git,
};

use crate::resources::absolute_git_hooks_path;

#[derive(Getters, Debug)]
pub struct Snapshot {
    is_git: bool,
    commit_msg_hook: Option<affirm_fs::File>,
    commit_rules: Option<affirm_fs::Directory>,
}

impl From<&Path> for Snapshot {
    fn from(value: &Path) -> Self {
        let is_git = is_git(value).unwrap();
        let main_dir = affirm_fs::Directory::try_from(value).unwrap();
        let commit_rules = main_dir.dir(COMMIT_RULES_NAME).map(|d| d.clone());
        let commit_msg_hook = if is_git {
            affirm_fs::File::try_from(
                absolute_git_hooks_path(value)
                    .unwrap()
                    .join(COMMIT_MSG_NAME),
            )
            .ok()
            .and_then(|x| x.take_and_hold_contents_as_static().ok())
        } else {
            None
        };

        Self {
            is_git,
            commit_msg_hook,
            commit_rules,
        }
    }
}
