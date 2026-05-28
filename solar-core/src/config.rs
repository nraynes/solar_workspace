use std::{fs, path::PathBuf};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{
    SolarError,
    tool::{CargoDeny, Commitalyzer, Licenses, PreCommit, SemverRelease, Vhooks, Workflows},
};

#[derive(Serialize, Deserialize, Debug, Getters)]
pub struct Config {
    vhooks: Option<Vhooks>,
    semver_release: Option<SemverRelease>,
    pre_commit: Option<PreCommit>,
    licenses: Option<Licenses>,
    github_workflows: Option<Workflows>,
    commitalyzer: Option<Commitalyzer>,
    cargo_deny: Option<CargoDeny>,
}

impl Config {
    /// Creates a new Config from a file at the supplied path, provided the file contains
    /// valid syntax for JSON and the config.
    pub fn load_from_file(file_path: PathBuf) -> Result<Self, SolarError> {
        let config_file = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_file)?;
        Ok(config)
    }
}
