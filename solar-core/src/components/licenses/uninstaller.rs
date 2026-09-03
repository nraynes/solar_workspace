use clap::Parser;
use derive_getters::Getters;
use derive_new::new;
use std::{
    fs::{self, DirEntry},
    path::Path,
    str::FromStr,
};

use crate::{
    components::licenses::{
        LICENSE_PREFIX, LICENSES_DIR, installation::Installation, license::License,
    },
    solar_error::SolarError,
    traits::{GetPartialInstall, Uninstallable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct LicensesUninstaller {
    /// Remove specific licenses included per dependency license agreements.
    #[arg(short, long, num_args = 0..)]
    include_licenses: Option<Vec<License>>,

    /// Remove licenses your project is licensed under currently.
    #[arg(short, long, num_args = 0..)]
    licensed_under: Option<Vec<License>>,
}

impl LicensesUninstaller {
    fn remove_all_licenses_in_dir(path: &Path) -> Result<(), SolarError> {
        if let Ok(read_dir) = fs::read_dir(path) {
            for dir_entry_result in read_dir {
                if let Ok(dir_entry) = dir_entry_result
                    && let Ok(file_name) = dir_entry.file_name().into_string()
                    && let Some((_, spdx_identifier)) = file_name.split_once(LICENSE_PREFIX)
                    && let Ok(license) = License::from_str(spdx_identifier.trim_end_matches("\n"))
                {
                    fs::remove_file(path.join(license.file_name()))?;
                }
            }
        }

        Ok(())
    }
}

impl Uninstallable for LicensesUninstaller {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let current_installation: Installation = Installation::get_current(path)?;

        // Set path variables.
        let licenses_dir = path.join(LICENSES_DIR);

        // If no arguments were given, remove all licenses.
        if self.include_licenses.is_none() && self.licensed_under.is_none() {
            Self::remove_all_licenses_in_dir(path)?;
            Self::remove_all_licenses_in_dir(&licenses_dir)?;
            if fs::read_dir(&licenses_dir).is_ok_and(|r| {
                r.filter_map(|d| d.ok())
                    .collect::<Vec<DirEntry>>()
                    .is_empty()
            }) {
                fs::remove_dir_all(&licenses_dir)?;
            }
        } else {
            // If include licenses were given, remove just those if they exist.
            if let Some(include_licenses) = &self.include_licenses
                && let Some(current_include_licenses) = current_installation.include_licenses()
            {
                for license in include_licenses {
                    if current_include_licenses.contains(license) {
                        fs::remove_file(licenses_dir.join(license.file_name()))?;
                    }
                }
            }

            // If project licenses were given, remove just those if they exist.
            if let Some(licensed_under) = &self.licensed_under {
                for license in licensed_under {
                    if current_installation.licensed_under().contains(license) {
                        fs::remove_file(path.join(license.file_name()))?;
                    }
                }
            }
        }

        Ok(())
    }
}
