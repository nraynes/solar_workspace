use std::path::Path;

use rust_dl::downloader::download_sync;
use url::Url;

use crate::{
    components::commitalyzer::{COMMIT_MSG_NAME, ruleset::Ruleset},
    match_target,
    solar_error::SolarError,
};

fn commitalyzer_binary_url() -> Result<Url, SolarError> {
    match_target!(
        Ok(Url::parse(
            "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/arm-macos/commit-msg",
        )?),
        Ok(Url::parse(
            "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/intel-macos/commit-msg",
        )?),
        Ok(Url::parse(
            "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/linux/commit-msg",
        )?),
        Ok(Url::parse(
            "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/windows/commit-msg",
        )?),
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
        "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/commit-rules/{}",
        ruleset.file_name()
    ))?;
    Ok(download_sync(
        download_url,
        destination.join(ruleset.file_name()),
    )?)
}
