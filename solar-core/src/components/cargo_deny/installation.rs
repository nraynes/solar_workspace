use std::{fs, path::Path};

use derive_getters::Getters;

use crate::{
    components::cargo_deny::{PKG_NAME, TOML_NAME},
    tools::cargo::is_crate_installed::is_crate_installed,
    solar_error::SolarError,
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
            crate_installed: is_crate_installed(PKG_NAME)?,
            deny_toml_exists: fs::exists(path.join(TOML_NAME))?,
        })
    }
}
