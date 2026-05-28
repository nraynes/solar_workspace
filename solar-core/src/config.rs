mod cargo_bin_basic;
mod cargo_lib_basic;
mod cargo_proc_basic;

pub use cargo_bin_basic::cargo_bin_basic;
pub use cargo_lib_basic::cargo_lib_basic;
pub use cargo_proc_basic::cargo_proc_basic;

use std::{fs, io::Write, path::PathBuf};

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
    pub fn new(
        vhooks: Option<Vhooks>,
        semver_release: Option<SemverRelease>,
        pre_commit: Option<PreCommit>,
        licenses: Option<Licenses>,
        github_workflows: Option<Workflows>,
        commitalyzer: Option<Commitalyzer>,
        cargo_deny: Option<CargoDeny>,
    ) -> Self {
        Self {
            vhooks,
            semver_release,
            pre_commit,
            licenses,
            github_workflows,
            commitalyzer,
            cargo_deny,
        }
    }

    /// Creates a new Config from a file at the supplied path, provided the file contains
    /// valid syntax for JSON and the config.
    pub fn load_from_file(file_path: PathBuf) -> Result<Self, SolarError> {
        let config_file = fs::read_to_string(file_path)?;
        let config: Config = serde_json::from_str(&config_file)?;
        Ok(config)
    }

    pub fn save_to_file(&self, file_path: PathBuf) -> Result<(), SolarError> {
        if !fs::exists(&file_path)? {
            fs::File::create(&file_path)?;
        }
        let mut file = fs::File::options().write(true).open(file_path)?;
        file.write_all(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }
}
