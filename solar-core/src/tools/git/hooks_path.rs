use std::path::{Path, PathBuf};

use derive_getters::Getters;

use crate::{
    solar_error::SolarError,
    tools::git::{DEFAULT_GIT_HOOKS_DIR, git_hooks_path},
};

#[derive(Getters)]
pub struct HooksPath {
    path: PathBuf,
    default: bool,
}

impl TryFrom<&Path> for HooksPath {
    type Error = SolarError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let hooks_path = git_hooks_path(value)?;
        let is_default = hooks_path
            .to_str()
            .ok_or("Could not convert hooks path to str.")?
            == DEFAULT_GIT_HOOKS_DIR;

        Ok(Self {
            path: hooks_path,
            default: is_default,
        })
    }
}
