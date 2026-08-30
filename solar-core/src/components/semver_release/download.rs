use std::{path::Path, str::FromStr};

use reqwest::blocking::Client;
use rust_dl::downloader::download_sync;
use serde_json::Value;
use url::Url;

use crate::{
    components::semver_release::{Platform, RELEASE_BIN_NAME},
    solar_error::SolarError,
    strings::{
        SEMVER_RELEASE_BINARY_URL_ARM_MACOS, SEMVER_RELEASE_BINARY_URL_X86_LINUX,
        SEMVER_RELEASE_BINARY_URL_X86_MACOS, SEMVER_RELEASE_BINARY_URL_X86_WINDOWS,
        SEMVER_RELEASE_CONFIG_URL,
    },
};

pub fn download_semver_release_binary(
    destination: &Path,
    platform: &Platform,
) -> Result<(), SolarError> {
    let download_url = Url::parse(match platform {
        Platform::ArmMacos => SEMVER_RELEASE_BINARY_URL_ARM_MACOS,
        Platform::X86Macos => SEMVER_RELEASE_BINARY_URL_X86_MACOS,
        Platform::X86Linux => SEMVER_RELEASE_BINARY_URL_X86_LINUX,
        Platform::X86Windows => SEMVER_RELEASE_BINARY_URL_X86_WINDOWS,
    })?;
    Ok(download_sync(
        download_url,
        destination.join(RELEASE_BIN_NAME),
    )?)
}

pub fn get_semver_release_config() -> Result<Value, SolarError> {
    let url = Url::parse(SEMVER_RELEASE_CONFIG_URL)?;
    let client = Client::new();
    let response = client.get(url).send()?;
    Ok(Value::from_str(&response.text()?)?)
}
