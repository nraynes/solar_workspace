use std::{
    fs,
    path::{Path, PathBuf},
};

use rust_terminal::Terminal;
use url::Url;

use crate::SolarError;

pub const SOLARCONFIGNAME: &str = "solar.config.json";

pub struct Global {}

macro_rules! match_target {
    ($macos_arm:expr, $macos_x:expr, $linux_x:expr, $windows_x:expr, $default:expr) => {
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        return $macos_arm;
        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        return $macos_x;
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        return $linux_x;
        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        return $windows_x;
        #[cfg(not(any(
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "windows", target_arch = "x86_64")
        )))]
        $default
    };
}

impl Global {
    /// Returns whether the path is a git repository or not.
    pub fn is_git(destination: &Path) -> Result<bool, SolarError> {
        Ok(fs::exists(destination.join(PathBuf::from(".git")))?)
    }

    /// Initialize a git repository at the destination if it's not already.
    pub fn git_init(destination: &Path) -> Result<(), SolarError> {
        if !Self::is_git(destination)? {
            Terminal::command()
                .current_dir(destination.to_path_buf())
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
        match_target!(
            Ok(Url::parse(
                "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/arm-macos/commit-msg",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/intel-macos/commit-msg",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/linux/commit-msg",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/bin/windows/commit-msg",
            )?),
            Err(SolarError::from("No download available for this target"))
        );
    }

    pub fn commitalyzer_conventional_commits_ruleset() -> Result<Url, SolarError> {
        Ok(Url::parse(
            "https://github.com/nraynes/commitalyzer/raw/refs/heads/master/commit-rules/conventional-commits.yml",
        )?)
    }

    pub fn semver_release_exec_download() -> Result<Url, SolarError> {
        match_target!(
            Ok(Url::parse(
                "https://github.com/nraynes/semver-release/raw/refs/heads/master/bin/arm-macos/semver-release",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/semver-release/raw/refs/heads/master/bin/intel-macos/semver-release",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/semver-release/raw/refs/heads/master/bin/linux/semver-release",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/semver-release/raw/refs/heads/master/bin/windows/semver-release.exe",
            )?),
            Err(SolarError::from("No download available for this target"))
        );
    }

    pub fn semver_release_config_url() -> Result<Url, SolarError> {
        Ok(Url::parse(
            "https://github.com/nraynes/semver-release/raw/refs/heads/master/sample.config.semver.json",
        )?)
    }

    pub fn semver_cargo_exec_download() -> Result<Url, SolarError> {
        match_target!(
            Ok(Url::parse(
                "https://github.com/nraynes/semver-cargo/raw/refs/heads/master/bin/arm-macos/semver-cargo",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/semver-cargo/raw/refs/heads/master/bin/intel-macos/semver-cargo",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/semver-cargo/raw/refs/heads/master/bin/linux/semver-cargo",
            )?),
            Ok(Url::parse(
                "https://github.com/nraynes/semver-cargo/raw/refs/heads/master/bin/windows/semver-cargo.exe",
            )?),
            Err(SolarError::from("No download available for this target"))
        );
    }
}
