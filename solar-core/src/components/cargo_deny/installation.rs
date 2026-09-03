use std::{fs, path::Path};

use derive_getters::Getters;

use crate::{
    components::cargo_deny::{CARGO_DENY_CRATE_NAME, DENY_TOML_NAME},
    solar_error::SolarError,
    tools::cargo::is_crate_installed::is_crate_installed,
    traits::GetPartialInstall,
};

#[derive(Getters)]
pub struct Installation {
    crate_installed: bool,
    deny_toml_exists: bool,
}

impl GetPartialInstall for Installation {
    fn get_current(path: &Path) -> Result<Self, SolarError> {
        Ok(Self {
            crate_installed: is_crate_installed(CARGO_DENY_CRATE_NAME)?,
            deny_toml_exists: fs::exists(path.join(DENY_TOML_NAME))?,
        })
    }
}
