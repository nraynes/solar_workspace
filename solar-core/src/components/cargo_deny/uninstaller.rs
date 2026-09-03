use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::cargo_deny::{DENY_TOML_NAME, installation::Installation},
    solar_error::SolarError,
    traits::{GetPartialInstall, Uninstallable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct CargoDenyUninstaller {}

impl Uninstallable for CargoDenyUninstaller {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let current_installation: Installation = Installation::get_current(path)?;

        // Remove configuration file if it exists.
        if *current_installation.deny_toml_exists() {
            fs::remove_file(path.join(DENY_TOML_NAME))?;
        }

        Ok(())
    }
}
