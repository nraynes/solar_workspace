use std::{path::Path, str::FromStr};

use reqwest::blocking::Client;
use rust_dl::downloader::download_sync;
use serde_json::Value;
use url::Url;

use crate::{components::semver_release::RELEASE_BIN_NAME, solar_error::SolarError};

pub fn download_semver_release_binary(destination: &Path) -> Result<(), SolarError> {
    let download_url = Url::parse(
        "https://github.com/nraynes/semver-release/raw/refs/heads/master/bin/arm-macos/semver-release",
    )?;
    Ok(download_sync(
        download_url,
        destination.join(RELEASE_BIN_NAME),
    )?)
}

pub fn get_semver_release_config() -> Result<Value, SolarError> {
    let url = Url::parse(
        "https://github.com/nraynes/semver-release/raw/refs/heads/master/sample.config.semver.json",
    )?;
    let client = Client::new();
    let response = client.get(url).send()?;
    Ok(Value::from_str(&response.text()?)?)
}
