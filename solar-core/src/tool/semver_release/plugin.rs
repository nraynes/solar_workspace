use std::{path::Path, str::FromStr};

use clap::ValueEnum;
use reqwest::blocking::Client;
use rust_dl::downloader::download_sync;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

use crate::{Global, SolarError};

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

    pub fn download_url(&self) -> Result<Url, SolarError> {
        match self {
            Self::Cargo => Global::semver_cargo_exec_download(),
        }
    }

    pub fn get_config(&self, client: &Client) -> Result<Map<String, Value>, SolarError> {
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

    pub fn download_exec(&self, download_path: &Path) -> Result<(), SolarError> {
        Ok(download_sync(
            self.download_url()?,
            download_path.join(self.bin_name()),
        )?)
    }
}
