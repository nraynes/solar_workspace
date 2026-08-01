use std::path::Path;

use rust_terminal::Terminal;

use crate::{components::git::is_git::is_git, solar_error::SolarError};

/// Initialize a git repository at the destination if it's not already.
pub fn try_git_init(path: &Path) -> Result<(), SolarError> {
    if !is_git(path)? {
        Terminal::command()
            .current_dir(path)
            .piped()
            .run("git", vec!["init"])?;
    }
    Ok(())
}

/// Initialize a git repository at the destination.
pub fn git_init(path: &Path) -> Result<(), SolarError> {
    Terminal::command()
        .current_dir(path)
        .piped()
        .run("git", vec!["init"])?;
    Ok(())
}
