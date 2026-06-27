mod ruleset;

use crate::{Config, Global, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
pub use ruleset::Ruleset;
use rust_dl::downloader::download_sync;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

static COMMIT_MSG_NAME: &str = "commit-msg";
static COMMIT_RULES_NAME: &str = "commit-rules";

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct Commitalyzer {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    /// The ruleset to install.
    #[arg(short, long)]
    ruleset: Option<Ruleset>,
}

impl Commitalyzer {
    pub fn new(destination: PathBuf, ruleset: Option<Ruleset>) -> Self {
        Self {
            destination,
            ruleset,
        }
    }
}

impl ToolTrait for Commitalyzer {
    fn set_dest(&mut self, dest: &Path) {
        self.destination = dest.to_path_buf();
    }

    fn install(&mut self) -> Result<(), SolarError> {
        let ruleset_name = self
            .ruleset
            .as_ref()
            .ok_or("A ruleset must be given for installation.")?;

        // Ensure working directory is a git repository.
        Global::git_init(&self.destination)?;

        // Update configuration file.
        let config = Config::load_or_default(&self.destination);
        config.set_commitalyzer(Some(self.clone())).save()?;

        // Download executable
        download_sync(
            Global::commitalyzer_exec_download()?,
            Global::git_hooks_path(&self.destination)?.join(COMMIT_MSG_NAME),
        )?;

        // Download commit rules
        let commit_rules_path = self.destination.join(COMMIT_RULES_NAME);
        fs::create_dir_all(&commit_rules_path)?;
        download_sync(
            Global::commitalyzer_conventional_commits_ruleset()?,
            commit_rules_path.join(format!("{}.yml", ruleset_name)),
        )?;

        Ok(())
    }

    fn upgrade(&mut self) -> Result<(), SolarError> {
        // Get configuration
        let config = Config::load_from(&self.destination)?;
        let config = config
            .commitalyzer()
            .clone()
            .ok_or("Cannot upgrade commitalyzer - commitalyzer not found in configuration.")?;

        self.ruleset = config.ruleset().clone();

        self.uninstall()?;
        self.install()
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        // Get configuration
        let config = Config::load_from(&self.destination)?;
        config
            .commitalyzer()
            .clone()
            .ok_or("Cannot uninstall commitalyzer - commitalyzer not found in configuration.")?;

        // Remove executable
        let exec_path = Global::git_hooks_path(&self.destination)?.join(COMMIT_MSG_NAME);
        if fs::exists(&exec_path)? {
            fs::remove_file(exec_path)?;
        }

        // Remove commit rules directory
        let commit_rules_path = self.destination.join(COMMIT_RULES_NAME);
        if fs::exists(&commit_rules_path)? {
            fs::remove_dir_all(commit_rules_path)?;
        }

        // Update configuration, remove if empty.
        let config = config.set_commitalyzer(None);
        match config.is_empty() {
            true => fs::remove_file(config.path())?,
            false => config.save()?,
        }

        Ok(())
    }
}
