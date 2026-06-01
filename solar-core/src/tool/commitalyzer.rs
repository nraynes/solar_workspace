use crate::{Global, SolarError, ToolTrait};
use clap::Parser;
use derive_getters::Getters;
use rust_dl::downloader::download_sync;
use rust_terminal::Terminal;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Parser, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Getters)]
pub struct Commitalyzer {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    #[serde(skip)]
    destination: PathBuf,
}

impl Commitalyzer {
    pub fn new(destination: PathBuf) -> Self {
        Self { destination }
    }

    fn commitmsg_path(&self) -> Result<PathBuf, SolarError> {
        let output = Terminal::command()
            .current_dir(self.destination.clone())
            .run("git", vec!["config", "core.hooksPath"])?;
        let git_hooks_path = PathBuf::from(String::from_utf8(output.stdout)?.trim());
        Ok(git_hooks_path.join(PathBuf::from("commit-msg")))
    }
}

impl ToolTrait for Commitalyzer {
    fn set_dest(&mut self, dest: PathBuf) {
        self.destination = dest;
    }

    fn install(&self) -> Result<(), SolarError> {
        // Download executable
        download_sync(
            Global::commitalyzer_exec_download()?,
            self.commitmsg_path()?,
        )?;

        // Download commit rules
        let commit_rules_path = self.destination.join(PathBuf::from("commit-rules"));
        fs::create_dir_all(&commit_rules_path)?;
        download_sync(
            Global::commitalyzer_conventional_commits_ruleset()?,
            commit_rules_path.join(PathBuf::from("conventional-commits.yml")),
        )?;
        Ok(())
    }

    fn uninstall(&self) -> Result<(), SolarError> {
        // Remove executable
        let exec_path = self.commitmsg_path()?;
        if fs::exists(&exec_path)? {
            fs::remove_dir_all(exec_path)?;
        }

        // Remove commit rules directory
        let commit_rules_path = self.destination.join(PathBuf::from("commit-rules"));
        if fs::exists(&commit_rules_path)? {
            fs::remove_dir_all(commit_rules_path)?;
        }
        Ok(())
    }
}
