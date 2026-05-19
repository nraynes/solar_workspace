use crate::{Global, SolarError, ToolTrait};
use clap::Parser;
use rust_dl::downloader::download_sync;
use rust_terminal::Terminal;
use std::{fs, path::PathBuf};

#[derive(Parser, Clone, Default, PartialEq, Debug)]
pub struct Commitalyzer {
    /// The working directory to use for installation.
    #[arg(short, long, default_value = ".")]
    working_dir: PathBuf,
}

impl Commitalyzer {
    fn commitmsg_path(&self) -> Result<PathBuf, SolarError> {
        let output = Terminal::command()
            .current_dir(self.working_dir.clone())
            .run("git", vec!["config", "core.hooksPath"])?;
        let git_hooks_path = PathBuf::from(String::from_utf8(output.stdout)?);
        Ok(self
            .working_dir
            .join(git_hooks_path.join(PathBuf::from("commit-msg"))))
    }
}

impl ToolTrait for Commitalyzer {
    fn install(&self) -> Result<(), SolarError> {
        // Download executable
        download_sync(
            Global::commitalyzer_exec_download()?,
            self.commitmsg_path()?,
        )?;

        // Download commit rules
        let commit_rules_path = self.working_dir.join(PathBuf::from("commit-rules"));
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
        let commit_rules_path = self.working_dir.join(PathBuf::from("commit-rules"));
        if fs::exists(&commit_rules_path)? {
            fs::remove_dir_all(commit_rules_path)?;
        }
        Ok(())
    }
}
