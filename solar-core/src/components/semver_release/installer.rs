use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;
use serde_json::Value;

use crate::{
    components::semver_release::{
        CONFIG_PLUGINS_SECTION, Plugin, RELEASE_CONFIG_NAME, RELEASE_DIR_NAME,
        download::{download_semver_release_binary, get_semver_release_config},
        installation::Installation,
    },
    solar_error::SolarError,
    traits::{GetPartialInstall, Installable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct SemverReleaseInstaller {
    /// The list of semver plugins to use.
    #[arg(short, long, num_args = 0..)]
    plugins: Option<Vec<Plugin>>,
}

impl Installable for SemverReleaseInstaller {
    fn install(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let mut current_installation: Installation = Installation::get_current(path)?;

        // Set path variables.
        let release_dir_path = path.join(RELEASE_DIR_NAME);

        // Make release directory if it does not exist.
        if !current_installation.release_dir() {
            fs::create_dir_all(&release_dir_path)?;
        }

        // Download main semver binary if it doesn't exist.
        if !current_installation.release_bin() {
            download_semver_release_binary(&release_dir_path)?;
        }

        // Download plugin binaries from arguments if they do not exist.
        if let Some(plugins) = self.plugins() {
            for plugin in plugins {
                if current_installation
                    .plugins()
                    .get(plugin.bin_name())
                    .is_none()
                {
                    if plugin.download_binary(&release_dir_path).is_ok() {
                        current_installation.add_plugin(plugin.to_owned());
                    }
                }
            }
        }

        // Get configuration sections for new plugins and currently existing plugins if they do not exist.
        let configuration = &mut current_installation
            .configuration
            .or(get_semver_release_config()
                .ok()
                .and_then(|v| v.as_object().and_then(|m| Some(m.to_owned()))))
            .ok_or("Failed to acquire semver configuration.")?;

        if let Some(plugin_configuration_value) = configuration.get_mut(CONFIG_PLUGINS_SECTION)
            && let Some(plugin_configuration) = plugin_configuration_value.as_object_mut()
        {
            for plugin in current_installation.plugins.values() {
                if plugin_configuration.get(plugin.bin_name()).is_none() {
                    plugin_configuration.insert(
                        plugin.bin_name().to_string(),
                        Value::from(plugin.get_config()?),
                    );
                }
            }
        }

        // Save the configuration file.
        let mut config_file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.join(RELEASE_CONFIG_NAME))?;
        config_file.write_all(serde_json::to_string_pretty(configuration)?.as_bytes())?;

        Ok(())
    }
}
