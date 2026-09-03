mod install;
mod uninstall;

use std::path::Path;

use derive_getters::Getters;
use solar_core::components::github_workflows::workflows_path;

#[derive(Getters, Debug)]
pub struct Snapshot {
    workflows_dir: Option<affirm_fs::Directory>,
}

impl From<&Path> for Snapshot {
    fn from(value: &Path) -> Self {
        Self {
            workflows_dir: affirm_fs::Directory::try_from(workflows_path(value))
                .ok()
                .and_then(|d| d.take_and_acquire_contents().ok()),
        }
    }
}
