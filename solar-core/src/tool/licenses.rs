use crate::{Global, SolarError, ToolTrait};
use clap::Parser;
use regex::Regex;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

static LICENSES_DIR: &str = "LICENSES";

fn default_licenses() -> Option<Vec<String>> {
    Some(vec!["MIT".to_string(), "Apache-2.0".to_string()])
}

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Licenses {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(default = "Global::default_destination")]
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
    fn get_license(&self, client: &Client, spdx: &str) -> Result<String, SolarError> {
        let response = client.get(Global::licenses_url(spdx)?).send()?;
        Ok(response.text()?)
    }
}

impl ToolTrait for Licenses {
    fn set_dest(&mut self, dest: PathBuf) {
        self.destination = dest;
    }

    fn install(&self) -> Result<(), SolarError> {
        let client = Client::new();
        let licenses_dir = self.destination.join(PathBuf::from(LICENSES_DIR));

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

        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        let pattern = Regex::new(r"^LICENSE-[_A-Za-z0-9\.\+-]+$")?;
        // Delete project license file.
        for entry in fs::read_dir(PathBuf::from("."))? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if pattern.is_match(
                        file_name
                            .to_str()
                            .ok_or(format!("Could not check pattern for license file."))?,
                    ) {
                        fs::remove_file(self.destination.join(path))?;
                    }
                }
            }
        }

        // Delete the licenses folder along with its contents.
        fs::remove_dir_all(self.destination.join(PathBuf::from(LICENSES_DIR)))?;
        Ok(())
    }
}
