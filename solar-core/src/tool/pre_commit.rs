use crate::{Config, Global, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

mod script;
pub use script::Script;

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct PreCommit {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,

    // The script to use as the pre-commit hook.
    #[arg(short, long)]
    script: Option<Script>,
}

impl PreCommit {
    pub fn new(destination: PathBuf, script: Option<Script>) -> Self {
        Self {
            destination,
            script,
        }
    }

    fn precommit_path(&self) -> Result<PathBuf, SolarError> {
        let git_hooks_path = Global::git_hooks_path(&self.destination)?;
        Ok(self.destination.join(git_hooks_path.join("pre-commit")))
    }
}

impl ToolTrait for PreCommit {
    fn set_dest(&mut self, dest: &Path) {
        self.destination = dest.to_path_buf();
    }

    fn install(&mut self) -> Result<(), SolarError> {
        let script = self.script.as_ref().ok_or("No script value provided")?;
        // Update configuration file.
        let config = Config::load_or_default(&self.destination);
        config.set_pre_commit(Some(self.clone())).save()?;

        // Ensure working directory is a git repository.
        Global::git_init(&self.destination)?;

        let mut precommit_file = Global::ovrwrt_file_or_default(&self.precommit_path()?)?;
        precommit_file.write_all(script.content().as_bytes())?;
        Ok(())
    }

    fn uninstall(&mut self) -> Result<(), SolarError> {
        // Get configuration file.
        let config = Config::load_or_default(&self.destination);

        // Remove pre-commit script.
        fs::remove_file(self.precommit_path()?)?;

        // Update configuration.
        let config = config.set_pre_commit(None);
        match config.is_empty() {
            true => fs::remove_file(config.path())?,
            false => config.save()?,
        }

        Ok(())
    }
}
