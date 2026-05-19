use std::env::consts::{ARCH, OS};
use std::{fs, path::PathBuf};

use rust_terminal::Terminal;
use url::Url;

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
            Terminal::command()
                .current_dir(destination.clone())
                .piped()
                .run("git", vec!["init"])?;
        }
        Ok(())
    }

    pub fn default_git_hook_dir() -> PathBuf {
        PathBuf::from(".git/hooks")
    }

    pub fn licenses_url(spdx: &str) -> Result<Url, SolarError> {
        Ok(Url::parse(&format!(
            "https://github.com/nraynes/licenses/raw/refs/heads/main/LICENSES/LICENSE-{}",
            spdx
        ))?)
    }

    pub fn commitalyzer_exec_download() -> Result<Url, SolarError> {
        let current_target = format!("{} {}", ARCH, OS);
        match current_target.as_str() {
            "aarch64 macos" => Ok(Url::parse(
                "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/arm-macos/commit-msg",
            )?),
            "x86_64 macos" => Ok(Url::parse(
                "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/intel-macos/commit-msg",
            )?),
            "x86_64 linux" => Ok(Url::parse(
                "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/linux/commit-msg",
            )?),
            "x86_64 windows" => Ok(Url::parse(
                "	https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/windows/commit-msg",
            )?),
            _ => Err(SolarError::from("No download available for this target")),
        }
    }

    pub fn commitalyzer_conventional_commits_ruleset() -> Result<Url, SolarError> {
        Ok(Url::parse(
            "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/commit-rules/conventional-commits.yml",
        )?)
    }
}
