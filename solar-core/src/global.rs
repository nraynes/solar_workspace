use std::{fs, path::PathBuf};

use rust_terminal::Terminal;

use crate::SolarError;

pub struct Global {}

impl Global {
    /// Returns whether the path is a git repository or not.
    pub fn is_git(destination: &PathBuf) -> bool {
        fs::exists(destination.join(PathBuf::from(".git"))).is_err()
    }

    /// Initialize a git repository at the destination if it's not already.
    pub fn git_init(destination: &PathBuf) -> Result<(), SolarError> {
        if Self::is_git(destination) {
            Terminal::command().piped().run("git", vec!["init"])?;
        }
        Ok(())
    }

    pub fn default_git_hook_dir() -> PathBuf {
        PathBuf::from(".git/hooks")
    }
}
