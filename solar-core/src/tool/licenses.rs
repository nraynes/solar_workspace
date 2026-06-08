use crate::{Config, Global, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

pub static LICENSES_DIR: &str = "LICENSES";

fn default_licenses() -> Option<Vec<String>> {
    Some(vec!["MIT".to_string(), "Apache-2.0".to_string()])
}

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct Licenses {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// The licenses to include in your project per conditions of dependency licenses.
    #[arg(short, long, default_values = ["MIT", "Apache-2.0"])]
    #[serde(default = "default_licenses")]
    include_licenses: Option<Vec<String>>,

    /// The licenses that the project will be licensed under.
    #[arg(short, long, default_values = ["MIT", "Apache-2.0"])]
    #[serde(default = "default_licenses")]
    licensed_under: Option<Vec<String>>,
}

impl Licenses {
    pub fn new(
        destination: PathBuf,
        include_licenses: Option<Vec<String>>,
        licensed_under: Option<Vec<String>>,
    ) -> Self {
        Self {
            destination,
            include_licenses,
            licensed_under,
        }
    }

    fn get_license(&self, client: &Client, spdx: &str) -> Result<String, SolarError> {
        let response = client.get(Global::licenses_url(spdx)?).send()?;
        Ok(response.text()?)
    }
}

impl ToolTrait for Licenses {
    fn set_dest(&mut self, dest: &Path) {
        self.destination = dest.to_path_buf();
    }

    fn install(&mut self) -> Result<(), SolarError> {
        let client = Client::new();
        let licenses_dir = self.destination.join(PathBuf::from(LICENSES_DIR));

        // Update configuration file.
        let config = Config::load_or_default(&self.destination);
        let current_tool_cfg = config.licenses().clone();
        config.set_licenses(Some(self.clone())).save()?;

        // Make a new licenses folder.
        fs::create_dir_all(&licenses_dir)?;

        // Add the included license files.
        if let Some(includes) = &self.include_licenses {
            for spdx in includes.iter() {
                let mut license_file =
                    File::create(licenses_dir.join(PathBuf::from(format!("LICENSE-{}", spdx))))?;
                let license_text = self.get_license(&client, spdx)?;
                license_file.write_all(license_text.as_bytes())?;
            }
        }

        // Add the project license files.
        if let Some(proj_licenses) = &self.licensed_under {
            for spdx in proj_licenses.iter() {
                let mut license_file = File::create(
                    self.destination
                        .join(PathBuf::from(format!("LICENSE-{}", spdx))),
                )?;
                let license_text = self.get_license(&client, spdx)?;
                license_file.write_all(license_text.as_bytes())?;
            }
        }

        // If there is a current installation, uninstall the old licenses.
        if let Some(mut licenses) = current_tool_cfg {
            licenses.set_dest(&self.destination);
            licenses.uninstall()?;
        }

        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        let config = Config::load_from(&self.destination)?;
        let current_tool_cfg: Self = config
            .licenses()
            .clone()
            .ok_or("Cannot uninstall licenses - vhooks not found in configuration.")?;

        let licenses_dir = self.destination.join(PathBuf::from(LICENSES_DIR));

        // Delete the included license files.
        if fs::exists(&licenses_dir)?
            && let Some(_) = &current_tool_cfg.include_licenses
        {
            fs::remove_dir_all(licenses_dir)?;
        }

        // Delete the project license files.
        if let Some(proj_licenses) = &self.licensed_under {
            for spdx in proj_licenses.iter() {
                let file_path = self
                    .destination
                    .join(PathBuf::from(format!("LICENSE-{}", spdx)));
                if fs::exists(&file_path)? {
                    fs::remove_file(file_path)?;
                }
            }
        }

        // Update configuration, remove if empty.
        let config = config.set_licenses(None);
        match config.is_empty() {
            true => fs::remove_file(config.path())?,
            false => config.save()?,
        }

        Ok(())
    }
}
