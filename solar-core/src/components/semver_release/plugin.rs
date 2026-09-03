use std::{path::Path, str::FromStr};

use clap::ValueEnum;
use reqwest::blocking::Client;
use rust_dl::downloader::download_sync;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

use crate::{
    components::semver_release::Platform,
    solar_error::SolarError,
    strings::{
        SEMVER_CARGO_BINARY_URL_ARM_MACOS, SEMVER_CARGO_BINARY_URL_X86_LINUX,
        SEMVER_CARGO_BINARY_URL_X86_MACOS, SEMVER_CARGO_BINARY_URL_X86_WINDOWS,
        SEMVER_CARGO_CONFIG_URL,
    },
};

#[derive(ValueEnum, Clone, PartialEq, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq)]
pub enum Plugin {
    SemverCargo,
}

impl Plugin {
    pub fn bin_name(&self) -> &str {
        match self {
            Self::SemverCargo => "semver-cargo",
        }
    }

    fn binary_download_url(&self, os: &Platform) -> Result<Url, SolarError> {
        Ok(match self {
            Self::SemverCargo => Url::parse(match os {
                Platform::ArmMacos => SEMVER_CARGO_BINARY_URL_ARM_MACOS,
                Platform::X86Macos => SEMVER_CARGO_BINARY_URL_X86_MACOS,
                Platform::X86Linux => SEMVER_CARGO_BINARY_URL_X86_LINUX,
                Platform::X86Windows => SEMVER_CARGO_BINARY_URL_X86_WINDOWS,
            })?,
        })
    }

    fn plugin_configuration_url(&self) -> Result<Url, SolarError> {
        Ok(match self {
            Self::SemverCargo => Url::parse(SEMVER_CARGO_CONFIG_URL)?,
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

    pub fn download_binary(&self, destination: &Path, os: &Platform) -> Result<(), SolarError> {
        Ok(download_sync(
            self.binary_download_url(os)?,
            destination.join(self.bin_name()),
        )?)
    }
}

impl FromStr for Plugin {
    type Err = SolarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cargo" => Ok(Self::SemverCargo),
            "semver-cargo" => Ok(Self::SemverCargo),
            _ => Err(SolarError::from(format!("{} is not a valid plugin.", s))),
        }
    }
}
