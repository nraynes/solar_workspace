use std::path::Path;

use rust_dl::downloader::download_sync;
use url::Url;

use crate::{components::licenses::license::License, solar_error::SolarError};

impl License {
    pub fn download_license(&self, destination: &Path) -> Result<(), SolarError> {
        let download_url = Url::parse(&format!(
            "https://github.com/nraynes/licenses/raw/refs/heads/main/LICENSES/LICENSE-{}",
            self
        ))?;
        Ok(download_sync(
            download_url,
            destination.join(self.file_name()),
        )?)
    }
}
