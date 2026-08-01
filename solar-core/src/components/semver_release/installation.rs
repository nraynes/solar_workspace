use derive_getters::Getters;

use std::{collections::HashMap, fs, path::Path, str::FromStr};

use serde_json::{Map, Value};

use crate::{
    components::semver_release::{Plugin, RELEASE_BIN_NAME, RELEASE_CONFIG_NAME, RELEASE_DIR_NAME},
    solar_error::SolarError,
    traits::GetPartialInstall,
};

#[derive(Getters)]
pub struct Installation {
    release_dir: bool,
    release_bin: bool,
    pub plugins: HashMap<String, Plugin>,
    pub configuration: Option<Map<String, Value>>,
}

impl Installation {
    fn release_directory_contents(path: &Path) -> Option<Vec<String>> {
        let dir_read = fs::read_dir(path.join(RELEASE_DIR_NAME)).ok()?;
        let mut dir_contents = Vec::new();
        for dir_result in dir_read {
            if let Ok(dir_entry) = dir_result
                && let Some(file_name) = dir_entry.file_name().to_str()
            {
                dir_contents.push(file_name.to_string());
            }
        }
        Some(dir_contents)
    }

    fn release_bin_exists(release_dir_contents: &Option<Vec<String>>) -> bool {
        release_dir_contents
            .as_ref()
            .is_some_and(|release_dir_file_names| {
                release_dir_file_names.contains(&RELEASE_BIN_NAME.to_string())
            })
    }

    fn semver_configuration(path: &Path) -> Result<Option<Value>, SolarError> {
        let config_file_text = fs::read_to_string(path.join(RELEASE_CONFIG_NAME))?;
        Ok(Some(Value::from_str(&config_file_text)?))
    }

    fn installed_plugins(
        release_dir_contents: &Option<Vec<String>>,
    ) -> Result<HashMap<String, Plugin>, SolarError> {
        let mut installed_plugins = HashMap::new();

        if let Some(release_dir_file_names) = release_dir_contents {
            for plugin_bin in release_dir_file_names {
                let plugin_result = Plugin::from_str(plugin_bin);
                if let Ok(plugin) = plugin_result {
                    installed_plugins.insert(plugin.bin_name().to_string(), plugin);
                }
            }
        }
        Ok(installed_plugins)
    }

    pub fn add_plugin(&mut self, plugin: Plugin) {
        self.plugins.insert(plugin.bin_name().to_string(), plugin);
    }
}

impl GetPartialInstall for Installation {
    fn get_current(path: &Path) -> Result<Self, SolarError> {
        let release_dir_contents = Self::release_directory_contents(path);
        let configuration_value = Self::semver_configuration(path)?;
        let configuration = configuration_value.and_then(|v| v.as_object().map(|m| m.to_owned()));
        Ok(Self {
            release_dir: release_dir_contents.is_some(),
            release_bin: Self::release_bin_exists(&release_dir_contents),
            plugins: Self::installed_plugins(&release_dir_contents)?,
            configuration,
        })
    }
}
