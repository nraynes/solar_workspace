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

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct Licenses {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// The licenses to include in your project per conditions of dependency licenses.
    #[arg(short, long, num_args = 0..)]
    include_licenses: Option<Vec<String>>,

    /// The licenses that the project will be licensed under.
    #[arg(short, long, num_args = 0..)]
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

    fn check_vec_eq_unord<T>(vec_one: &Vec<T>, vec_two: &Vec<T>) -> bool
    where
        T: Ord + Clone,
    {
        let mut vec_one_copy = vec_one.clone();
        let mut vec_two_copy = vec_two.clone();
        vec_one_copy.sort();
        vec_two_copy.sort();
        vec_one_copy == vec_two_copy
    }

    fn check_opt_vec_eq_unord<T>(opt_one: &Option<Vec<T>>, opt_two: &Option<Vec<T>>) -> bool
    where
        T: Ord + Clone,
    {
        if let Some(vec_one) = opt_one
            && let Some(vec_two) = opt_two
        {
            return Self::check_vec_eq_unord(vec_one, vec_two);
        }
        opt_one == opt_two
    }

    fn update_config_after_uninstall(&self, current: &Self) -> Option<Self> {
        // Gather the include licenses that were not removed.
        let mut new_includes: Vec<String> = Vec::new();
        if let Some(current_includes) = current.include_licenses()
            && let Some(removed_includes) = &self.include_licenses
        {
            for include in current_includes {
                if !removed_includes.contains(include) {
                    new_includes.push(include.clone())
                }
            }
        }

        // Gather the project licenses that were not removed.
        let mut new_under: Vec<String> = Vec::new();
        if let Some(current_under) = current.licensed_under()
            && let Some(removed_under) = &self.licensed_under
        {
            for under in current_under {
                if !removed_under.contains(under) {
                    new_under.push(under.clone())
                }
            }
        }

        // Return new licenses configuration.
        if new_includes.len() <= 0 && new_under.len() <= 0 {
            return None;
        }
        Some(Self::new(
            self.destination.clone(),
            Some(new_includes),
            Some(new_under),
        ))
    }

    fn get_license(client: &Client, spdx: &str) -> Result<String, SolarError> {
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

        // Get or make configuration file.
        let config = Config::load_or_default(&self.destination);

        // Ensure license folder exists.
        fs::create_dir_all(&licenses_dir)?;

        if let Some(cmd_include_licenses) = &mut self.include_licenses {
            // Add the included licenses to configuration.
            if let Some(current_config) = config.licenses()
                && let Some(cfg_include_licenses) = current_config.include_licenses()
            {
                cmd_include_licenses.extend(cfg_include_licenses.clone());
            }

            // Add the included license files.
            for spdx in cmd_include_licenses.iter() {
                let license_path = licenses_dir.join(PathBuf::from(format!("LICENSE-{}", spdx)));
                if !fs::exists(&license_path)? {
                    let mut license_file = File::create(license_path)?;
                    let license_text = Self::get_license(&client, spdx)?;
                    license_file.write_all(license_text.as_bytes())?;
                }
            }
        }

        if let Some(cmd_licensed_under) = &mut self.licensed_under {
            // Add the project licenses to configuration.
            if let Some(current_config) = config.licenses()
                && let Some(cfg_licensed_under) = current_config.licensed_under()
            {
                cmd_licensed_under.extend(cfg_licensed_under.clone());
            }

            // Add the project license files.
            for spdx in cmd_licensed_under.iter() {
                let license_path = self
                    .destination
                    .join(PathBuf::from(format!("LICENSE-{}", spdx)));
                if !fs::exists(&license_path)? {
                    let mut license_file = File::create(license_path)?;
                    let license_text = Self::get_license(&client, spdx)?;
                    license_file.write_all(license_text.as_bytes())?;
                }
            }
        }

        config.set_licenses(Some(self.clone())).save()?;

        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        let config = Config::load_from(&self.destination)?;
        let current_config = config
            .licenses()
            .as_ref()
            .ok_or("No licenses configuration inside config file.")?;

        // If no args are supplied, default to removing all licenses in configuration.
        if self.include_licenses == None && self.licensed_under == None {
            self.include_licenses = current_config.include_licenses().clone();
            self.licensed_under = current_config.licensed_under().clone();
        }

        let licenses_dir = self.destination.join(PathBuf::from(LICENSES_DIR));

        // Delete the included license files.
        if fs::exists(&licenses_dir)? {
            if Self::check_opt_vec_eq_unord(
                &current_config.include_licenses(),
                &self.include_licenses,
            ) {
                fs::remove_dir_all(licenses_dir)?;
            } else {
                if let Some(includes) = &self.include_licenses {
                    for spdx in includes.iter() {
                        let file_path = licenses_dir.join(format!("LICENSE-{}", spdx));
                        if fs::exists(&file_path)? {
                            fs::remove_file(file_path)?;
                        }
                    }
                }
            }
        }

        // Delete the project license files.
        if let Some(proj_licenses) = &self.licensed_under {
            for spdx in proj_licenses.iter() {
                let file_path = self.destination.join(format!("LICENSE-{}", spdx));
                if fs::exists(&file_path)? {
                    fs::remove_file(file_path)?;
                }
            }
        }

        // Update configuration, remove if empty.
        let new_licenses_config = self.update_config_after_uninstall(current_config);
        let config = config.set_licenses(new_licenses_config);
        match config.is_empty() {
            true => fs::remove_file(config.path())?,
            false => config.save()?,
        }

        Ok(())
    }
}
