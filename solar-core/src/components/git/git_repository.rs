use std::path::Path;

use derive_getters::Getters;

use crate::{components::git::is_git::is_git, solar_error::SolarError, traits::GetPartialInstall};

#[derive(Getters)]
pub struct GitRepository<T: GetPartialInstall> {
    installation: T,
}

impl<T: GetPartialInstall> GitRepository<T> {
    pub fn from_path(path: &Path) -> Result<Self, SolarError> {
        match is_git(path)? {
            true => Ok(Self {
                installation: T::get_current(path)?,
            }),
            false => Err(SolarError::from(format!(
                "The directory at path {:?} is not a git repository.",
                path
            ))),
        }
    }
}
