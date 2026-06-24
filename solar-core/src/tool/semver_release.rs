use crate::{Config, Global, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
use reqwest::blocking::Client;
use rust_dl::downloader::download_sync;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
mod plugin;

pub use plugin::Plugin;

pub static RELEASE_DIR_NAME: &str = ".release";
pub static RELEASE_BIN_NAME: &str = "semver-release";
pub static RELEASE_CONFIG_NAME: &str = "config.semver.json";

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct SemverRelease {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// The list of semver plugins to use.
    #[arg(short, long, num_args = 0..)]
    plugins: Option<Vec<Plugin>>,
}

impl SemverRelease {
    pub fn new(destination: PathBuf, plugins: Option<Vec<Plugin>>) -> Self {
        Self {
            destination,
            plugins,
        }
    }
}

impl ToolTrait for SemverRelease {
    fn set_dest(&mut self, dest: &Path) {
        self.destination = dest.to_path_buf();
    }

    fn install(&mut self) -> Result<(), SolarError> {
        let client = Client::new();

        // Make or get configuration file.
        let config = Config::load_or_default(&self.destination);

        // Track current configuration plugins.
        if let Some(semver_config) = config.semver_release()
            && let Some(config_plugins) = semver_config.plugins()
        {
            match &mut self.plugins {
                Some(arg_plugins) => arg_plugins.extend(config_plugins.clone()),
                None => self.plugins = Some(config_plugins.clone()),
            }
        }
        if let Some(plugins) = &mut self.plugins {
            plugins.sort_unstable();
            plugins.dedup();
        }

        // Make release directory
        let release_dir_path = self.destination.join(RELEASE_DIR_NAME);
        fs::create_dir_all(&release_dir_path)?;

        // Download executable
        download_sync(
            Global::semver_release_exec_download()?,
            release_dir_path.join(RELEASE_BIN_NAME),
        )?;

        // Download config and plugins
        let response = client.get(Global::semver_release_config_url()?).send()?;
        let mut release_config = Value::from_str(&response.text()?)?;
        let plugin_section = release_config
            .as_object_mut()
            .ok_or("Could not parse release config.")?
            .get_mut("plugins")
            .ok_or("Could not extract plugins config.")?
            .as_object_mut()
            .ok_or("Could not parse plugins config.")?;
        if let Some(plugins) = &self.plugins {
            for plugin in plugins {
                plugin.download_exec(&release_dir_path)?;
                let plugin_config = plugin.get_config(&client)?;
                plugin_section.extend(plugin_config);
            }
        }

        // Save the configuration file.
        let release_config_text = release_config.to_string();
        let config_path = self.destination.join(RELEASE_CONFIG_NAME);
        if !fs::exists(&config_path)? {
            File::create(&config_path)?;
        }
        let mut config_file = File::options()
            .write(true)
            .truncate(true)
            .open(&config_path)?;
        config_file.write_all(release_config_text.as_bytes())?;

        // Save the solar configuration file.
        config.set_semver_release(Some(self.clone())).save()?;

        Ok(())
    }

    fn upgrade(&mut self) -> Result<(), SolarError> {
        self.install()
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        // Get configuration
        let config = Config::load_from(&self.destination)?;
        let semver_config = config.semver_release().as_ref().ok_or(
            "Cannot uninstall semver_release - semver_release not found in configuration.",
        )?;

        let release_dir_path = self.destination.join(RELEASE_DIR_NAME);
        let config_path = self.destination.join(RELEASE_CONFIG_NAME);

        // Remove individual plugins if given, otherwise remove all.
        match &self.plugins {
            Some(plugins) => {
                let config_text = fs::read_to_string(&config_path)?;
                let mut release_config = Value::from_str(&config_text)?;
                let plugin_section = release_config
                    .as_object_mut()
                    .ok_or("Could not parse release config.")?
                    .get_mut("plugins")
                    .ok_or("Could not extract plugins config.")?
                    .as_object_mut()
                    .ok_or("Could not parse plugins config.")?;

                let mut new_plugins = Vec::new();
                for plugin in plugins {
                    // Remove bin.
                    let plugin_path = release_dir_path.join(plugin.bin_name());
                    if fs::exists(&plugin_path)? {
                        fs::remove_file(plugin_path)?;
                    }

                    // Remove plugin configuration.
                    if plugin_section.contains_key(plugin.bin_name()) {
                        plugin_section.remove_entry(plugin.bin_name());
                    }

                    // Remove from solar configuration.
                    if let Some(current_plugins) = semver_config.plugins() {
                        if !current_plugins.contains(plugin) {
                            new_plugins.push(plugin.clone());
                        }
                    }
                }

                // Save new semver configuration.
                let release_config_text = release_config.to_string();
                let mut config_file = File::options()
                    .write(true)
                    .truncate(true)
                    .open(&config_path)?;
                config_file.write_all(release_config_text.as_bytes())?;

                // Save solar configuration.
                self.plugins = match new_plugins.is_empty() {
                    true => None,
                    false => Some(new_plugins),
                };
                let config = config.set_semver_release(Some(self.clone()));
                config.save()?;
            }
            None => {
                // Remove release directory
                fs::remove_dir_all(&release_dir_path)?;

                // Remove config
                if fs::exists(&config_path)? {
                    fs::remove_file(&config_path)?;
                }

                // Update configuration, remove if empty.
                let config = config.set_semver_release(None);
                match config.is_empty() {
                    true => fs::remove_file(config.path())?,
                    false => config.save()?,
                }
            }
        }

        Ok(())
    }
}
