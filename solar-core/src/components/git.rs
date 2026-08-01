use std::path::{Path, PathBuf};

use rust_terminal::Terminal;

use crate::solar_error::SolarError;

mod git_repository;
pub mod gitignore;
mod hooks_path;
pub mod init;
mod installer;
pub mod is_git;
mod uninstaller;

pub use git_repository::GitRepository;
pub use hooks_path::HooksPath;

pub static DEFAULT_GIT_HOOKS_DIR: &str = ".git/hooks";

pub fn git_hooks_path(path: &Path) -> Result<PathBuf, SolarError> {
    let command_output = Terminal::command()
        .current_dir(path)
        .run("git", ["config", "core.hooksPath"])?;
    let output_text = String::from_utf8(command_output.stdout)?;
    if output_text.is_empty() {
        return Ok(PathBuf::from(DEFAULT_GIT_HOOKS_DIR));
    }
    Ok(PathBuf::from(output_text.trim()))
}
