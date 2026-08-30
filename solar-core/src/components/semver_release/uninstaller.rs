use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    components::semver_release::{
        CONFIG_PLUGINS_SECTION, Plugin, RELEASE_CONFIG_NAME, RELEASE_DIR_NAME,
        installation::Installation,
    },
    solar_error::SolarError,
    traits::{GetPartialInstall, Uninstallable},
};

#[derive(Parser, Clone, Default, PartialEq, Debug, Getters, new)]
pub struct SemverReleaseUninstaller {
    /// The list of semver plugins to uninstall.
    #[arg(short, long, num_args = 0..)]
    plugins: Option<Vec<Plugin>>,
}

impl Uninstallable for SemverReleaseUninstaller {
    fn uninstall(&self, path: &Path) -> Result<(), SolarError> {
        // Get current installation if it exists.
        let mut current_installation: Installation = Installation::get_current(path)?;

        // Set path variables.
        let release_dir_path = path.join(RELEASE_DIR_NAME);
        let config_path = path.join(RELEASE_CONFIG_NAME);

        // If individual plugins were supplied as args, just remove those plugins. Otherwise, remove installation.
        match &self.plugins {
            Some(plugins) => {
                if let Some(configuration) = &mut current_installation.configuration
                    && let Some(plugin_configuration_value) =
                        configuration.get_mut(CONFIG_PLUGINS_SECTION)
                    && let Some(plugin_configuration) = plugin_configuration_value.as_object_mut()
                {
                    for plugin in plugins {
                        let plugin_path = release_dir_path.join(plugin.bin_name());
                        if let Err(e) = fs::remove_file(&plugin_path)
                            && let Some(path_str) = plugin_path.to_str()
                        {
                            println!(
                                "There was a problem removing the file at {}: {}",
                                path_str, e
                            );
                        };
                        if plugin_configuration.remove(plugin.bin_name()).is_none() {
                            println!(
                                "Skipping removing configuration for plugin {}. Plugin configuration does not exist.",
                                plugin.bin_name()
                            );
                        }
                    }

                    let mut config_file = File::options()
                        .write(true)
                        .truncate(true)
                        .open(&config_path)?;
                    config_file
                        .write_all(serde_json::to_string_pretty(configuration)?.as_bytes())?;
                }
            }
            None => {
                if *current_installation.release_dir() {
                    fs::remove_dir_all(&release_dir_path)?;
                }
                if current_installation.configuration().is_some() {
                    fs::remove_file(&config_path)?;
                }
            }
        }

        Ok(())
    }
}
