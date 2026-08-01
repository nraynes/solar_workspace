use std::path::Path;

use derive_getters::Getters;

use crate::{components::git::HooksPath, solar_error::SolarError, traits::GetPartialInstall};

#[derive(Getters)]
pub struct Installation {
    hooks_path: HooksPath,
}

impl GetPartialInstall for Installation {
    fn get_current(path: &Path) -> Result<Self, SolarError> {
        Ok(Self {
            hooks_path: HooksPath::try_from(path)?,
        })
    }
}
