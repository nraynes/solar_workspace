use std::{fs, path::Path};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::licenses::{LICENSES_DIR, installation::Installation, license::License},
    solar_error::SolarError,
    traits::{GetPartialInstall, Installable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct LicensesInstaller {
    /// Optional licenses to include in your project per conditions of dependency licenses.
    #[arg(short, long, num_args = 0..)]
    include_licenses: Option<Vec<License>>,

    /// The licenses that the project will be licensed under.
    #[arg(short, long, num_args = 0..)]
    licensed_under: Vec<License>,
}

impl Installable for LicensesInstaller {
    fn install(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let current_installation: Installation = Installation::get_current(path)?;

        // Set path variables.
        let licenses_dir = path.join(LICENSES_DIR);

        // Create include licenses directory if it does not exist.
        if current_installation.include_licenses().is_none() {
            fs::create_dir_all(&licenses_dir)?;
        }

        // Add include license files if given, and they do not currently exist.
        if let Some(include_licenses) = &self.include_licenses {
            for license in include_licenses {
                if current_installation
                    .include_licenses()
                    .as_ref()
                    .is_none_or(|l| !l.contains(&license))
                {
                    license.download_license(&licenses_dir)?;
                }
            }
        }

        // Add license files to project that do not currently exist.
        for license in &self.licensed_under {
            if !current_installation.licensed_under().contains(&license) {
                license.download_license(path)?;
            }
        }

        Ok(())
    }
}
