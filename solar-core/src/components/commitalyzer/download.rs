use std::path::Path;

use rust_dl::downloader::download_sync;
use solar_utils::match_target;
use url::Url;

use crate::{
    components::commitalyzer::{COMMIT_MSG_NAME, ruleset::Ruleset},
    solar_error::SolarError,
    strings::{
        COMMITALYZER_BINARY_URL_ARM_MACOS, COMMITALYZER_BINARY_URL_X86_LINUX,
        COMMITALYZER_BINARY_URL_X86_MACOS, COMMITALYZER_BINARY_URL_X86_WINDOWS,
        COMMITALYZER_RULESET_BASE_URL,
    },
};

fn commitalyzer_binary_url() -> Result<Url, SolarError> {
    match_target!(
        Ok(Url::parse(COMMITALYZER_BINARY_URL_ARM_MACOS,)?),
        Ok(Url::parse(COMMITALYZER_BINARY_URL_X86_MACOS,)?),
        Ok(Url::parse(COMMITALYZER_BINARY_URL_X86_LINUX,)?),
        Ok(Url::parse(COMMITALYZER_BINARY_URL_X86_WINDOWS,)?),
        Err(SolarError::from("No download available for this target"))
    );
}

pub fn download_commitalyzer_binary(destination: &Path) -> Result<(), SolarError> {
    let download_url = commitalyzer_binary_url()?;
    Ok(download_sync(
        download_url,
        destination.join(COMMIT_MSG_NAME),
    )?)
}

pub fn download_commitalyzer_ruleset(
    destination: &Path,
    ruleset: &Ruleset,
) -> Result<(), SolarError> {
    let download_url = Url::parse(&format!(
        "{}/{}",
        COMMITALYZER_RULESET_BASE_URL,
        ruleset.file_name()
    ))?;
    Ok(download_sync(
        download_url,
        destination.join(ruleset.file_name()),
    )?)
}
