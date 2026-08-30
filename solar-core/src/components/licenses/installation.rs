use std::{fs, path::Path, str::FromStr};

use derive_getters::Getters;

use crate::{
    components::licenses::{LICENSES_DIR, license::License},
    solar_error::SolarError,
    traits::GetPartialInstall,
};

#[derive(Getters)]
pub struct Installation {
    include_licenses: Option<Vec<License>>,
    licensed_under: Vec<License>,
}

impl Installation {
    fn get_licenses_in_dir(path: &Path) -> Option<Vec<License>> {
        if let Ok(read_dir) = fs::read_dir(path) {
            let mut include_licenses = Vec::new();
            for dir_entry_result in read_dir {
                if let Ok(dir_entry) = dir_entry_result
                    && let Ok(file_name) = dir_entry.file_name().into_string()
                    && let Some((_, spdx_identifier)) = file_name.split_once("LICENSE-")
                    && let Ok(license) = License::from_str(spdx_identifier)
                {
                    include_licenses.push(license);
                }
            }
            return Some(include_licenses);
        }
        None
    }
}

impl GetPartialInstall for Installation {
    fn get_current(path: &Path) -> Result<Self, SolarError> {
        Ok(Self {
            include_licenses: Self::get_licenses_in_dir(&path.join(LICENSES_DIR)),
            licensed_under: Self::get_licenses_in_dir(path).unwrap_or_default(),
        })
    }
}
