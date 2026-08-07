use std::{fs::File, io::Write, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::cargo_deny::{
        PKG_NAME, TOML_NAME, generate_toml::generate_toml, installation::Installation,
    },
    tools::cargo::try_cargo_install::try_cargo_install,
    solar_error::SolarError,
    traits::{GetPartialInstall, Installable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct CargoDenyInstaller {
    /// Default licenses to allow in your dependencies in your project.
    #[arg(short, long, num_args = 0..)]
    allow_licenses: Vec<String>,
}

impl Installable for CargoDenyInstaller {
    fn install(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let current_installation: Installation = Installation::get_current(path)?;

        // Check for current toml configuration.
        if *current_installation.deny_toml_exists() {
            return Err(SolarError::from(
                "Current installation found. Use cargo-deny to make changes to the current installation. Only use cargo-solar to uninstall completely.",
            ));
        }

        // Ensure that tool is globally installed.
        if !current_installation.crate_installed() {
            try_cargo_install(PKG_NAME)?;
        }

        // Generate config file.
        let toml_contents = generate_toml(&self.allow_licenses)?.into_bytes();

        // Create configuration file.
        let mut deny_config = File::create(path.join(TOML_NAME))?;
        deny_config.write_all(&toml_contents)?;

        Ok(())
    }
}
