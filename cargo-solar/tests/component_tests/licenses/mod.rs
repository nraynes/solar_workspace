mod install;
mod uninstall;

use std::path::Path;

use affirm_fs::DirStructure;
use derive_getters::Getters;
use solar_core::components::licenses::{LICENSE_PREFIX, LICENSES_DIR};

#[derive(Getters, Debug)]
pub struct Snapshot {
    include_licenses: Option<affirm_fs::Directory>,
    licensed_under: affirm_fs::Directory,
}

impl From<&Path> for Snapshot {
    fn from(value: &Path) -> Self {
        let main_dir = affirm_fs::Directory::try_from(value).unwrap();
        let include_licenses = main_dir.dir(LICENSES_DIR).map(|d| d.clone());
        let mut licensed_under = DirStructure::new(value);
        for file in main_dir.files().values() {
            if let Some(license_filename_os_str) = file.path().file_name() {
                let license_filename = license_filename_os_str.to_str().unwrap();
                if license_filename.starts_with(LICENSE_PREFIX) {
                    licensed_under = licensed_under.file(license_filename);
                }
            }
        }

        Self {
            include_licenses,
            licensed_under: licensed_under.build(),
        }
    }
}
