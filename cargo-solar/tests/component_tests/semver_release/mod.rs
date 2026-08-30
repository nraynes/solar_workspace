use std::{fs, path::Path};

use affirm_fs::Directory;
use derive_getters::Getters;
use serde_json::{Map, Value};
use solar_core::components::semver_release::{RELEASE_CONFIG_NAME, RELEASE_DIR_NAME};

mod install;
mod uninstall;
mod upgrade;

#[derive(Getters)]
pub struct Snapshot {
    release_dir: Option<Directory>,
    semver_config: Option<Map<String, Value>>,
}

impl From<&Path> for Snapshot {
    fn from(value: &Path) -> Self {
        let config_path = value.join(RELEASE_CONFIG_NAME);
        let release_dir = Directory::try_from(value.join(RELEASE_DIR_NAME)).ok();
        let semver_config = fs::exists(&config_path).unwrap().then(|| {
            serde_json::from_str::<Value>(&fs::read_to_string(&config_path).unwrap())
                .unwrap()
                .as_object()
                .unwrap()
                .clone()
        });

        Self {
            release_dir,
            semver_config,
        }
    }
}

impl Snapshot {
    pub fn plugin_configurations(&self) -> Option<&Map<String, Value>> {
        self.semver_config
            .as_ref()
            .and_then(|x| x.get("plugins").and_then(|x| x.as_object()))
    }
}
