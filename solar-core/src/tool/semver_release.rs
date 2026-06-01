use crate::{Global, SolarError, ToolTrait};
use clap::{Parser, ValueEnum};
use derive_getters::Getters;
use reqwest::blocking::Client;
use rust_dl::downloader::download_sync;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use url::Url;

#[derive(ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Plugin {
    Cargo,
}

impl Plugin {
    fn get_config(&self, client: &Client) -> Result<Map<String, Value>, SolarError> {
        let url = match self {
            Self::Cargo => Url::parse(
                "https://github.com/nraynes/semver-cargo/raw/refs/heads/master/sample.plugin.config.json",
            )?,
        };
        let response = client.get(url).send()?;
        let value = &Value::from_str(&response.text()?)?;
        Ok(value
            .as_object()
            .ok_or("Could not parse plugin config.")?
            .clone())
    }

    fn download_exec(&self, download_path: &Path) -> Result<(), SolarError> {
        match self {
            Self::Cargo => download_sync(
                Global::semver_cargo_exec_download()?,
                download_path.join(PathBuf::from("semver-cargo")),
            )?,
        }
        Ok(())
    }
}

fn default_plugins() -> Vec<Plugin> {
    vec![Plugin::Cargo]
}

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct SemverRelease {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// The list of semver plugins to use.
    #[arg(short, long, default_values = ["cargo"])]
    #[serde(default = "default_plugins")]
    plugins: Vec<Plugin>,
}

impl SemverRelease {
    pub fn new(destination: PathBuf, plugins: Vec<Plugin>) -> Self {
        Self {
            destination,
            plugins,
        }
    }
}

impl ToolTrait for SemverRelease {
    fn set_dest(&mut self, dest: PathBuf) {
        self.destination = dest;
    }

    fn install(&mut self) -> Result<(), SolarError> {
        let client = Client::new();

        // Make release directory
        let release_dir_path = self.destination.join(PathBuf::from(".release"));
        fs::create_dir_all(&release_dir_path)?;

        // Download executable
        download_sync(
            Global::semver_release_exec_download()?,
            release_dir_path.join(PathBuf::from("semver-release")),
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
        for plugin in &self.plugins {
            plugin.download_exec(&release_dir_path)?;
            let plugin_config = plugin.get_config(&client)?;
            plugin_section.extend(plugin_config);
        }
        let release_config_text = release_config.to_string();
        let mut config_file =
            File::create(self.destination.join(PathBuf::from("config.semver.json")))?;
        config_file.write_all(release_config_text.as_bytes())?;
        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        // Remove release directory
        let release_dir_path = self.destination.join(PathBuf::from(".release"));
        fs::remove_dir_all(&release_dir_path)?;

        // Remove config
        let config_path = self.destination.join(PathBuf::from("config.semver.json"));
        if fs::exists(&config_path)? {
            fs::remove_file(&config_path)?;
        }

        Ok(())
    }
}
