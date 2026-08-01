use std::{path::Path, str::FromStr};

use clap::ValueEnum;
use reqwest::blocking::Client;
use rust_dl::downloader::download_sync;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

use crate::solar_error::SolarError;

#[derive(ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq)]
pub enum Plugin {
    Cargo,
}

impl Plugin {
    pub fn bin_name(&self) -> &str {
        match self {
            Self::Cargo => "semver-cargo",
        }
    }

    fn binary_download_url(&self) -> Result<Url, SolarError> {
        Ok(match self {
            Self::Cargo => Url::parse(
                "https://github.com/nraynes/semver-cargo/raw/refs/heads/master/bin/arm-macos/semver-cargo",
            )?,
        })
    }

    fn plugin_configuration_url(&self) -> Result<Url, SolarError> {
        Ok(match self {
            Self::Cargo => Url::parse(
                "https://github.com/nraynes/semver-cargo/raw/refs/heads/master/sample.plugin.config.json",
            )?,
        })
    }

    pub fn get_config(&self) -> Result<Map<String, Value>, SolarError> {
        let client = Client::new();
        let response = client.get(self.plugin_configuration_url()?).send()?;
        let value = &Value::from_str(&response.text()?)?;
        Ok(value
            .as_object()
            .ok_or("Could not parse plugin config.")?
            .clone())
    }

    pub fn download_binary(&self, destination: &Path) -> Result<(), SolarError> {
        Ok(download_sync(
            self.binary_download_url()?,
            destination.join(self.bin_name()),
        )?)
    }
}

impl FromStr for Plugin {
    type Err = SolarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(Self::Cargo),
            "semver-cargo" => Ok(Self::Cargo),
            _ => Err(SolarError::from(format!("{} is not a valid plugin.", s))),
        }
    }
}
