use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::semver_release::{
        Platform, RELEASE_BIN_NAME, RELEASE_DIR_NAME, download::download_semver_release_binary,
        installation::Installation,
    },
    solar_error::SolarError,
    traits::{GetPartialInstall, Upgradable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct SemverReleaseUpgrader {
    /// The desired platform for the binary.
    #[arg(short, long, default_value = "arm-macos")]
    os: Platform,
}

impl Upgradable for SemverReleaseUpgrader {
    fn upgrade(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let current_installation: Installation = Installation::get_current(path)?;

        // Set path variables.
        let release_dir_path = path.join(RELEASE_DIR_NAME);

        // Upgrade release binary if it exists.
        if *current_installation.release_bin()
            && fs::remove_file(release_dir_path.join(RELEASE_BIN_NAME)).is_ok()
            && let Err(e) = download_semver_release_binary(&release_dir_path, &self.os)
        {
            println!(
                "There was an error while trying to upgrade semver-release main binary.\n\nERROR: {}",
                e
            );
        };

        // Upgrade plugin binaries if they exist.
        for plugin in current_installation.plugins().values() {
            if fs::remove_file(release_dir_path.join(plugin.bin_name())).is_ok()
                && let Err(e) = plugin.download_binary(&release_dir_path, &self.os)
            {
                println!(
                    "There was an error while trying to upgrade semver plugin {}.\n\nERROR: {}",
                    plugin.bin_name(),
                    e
                );
            };
        }

        Ok(())
    }
}
