use std::path::Path;

use rust_terminal::Terminal;

use crate::solar_error::SolarError;

/// Initialize a git repository at the destination.
pub fn git_init(path: &Path) -> Result<(), SolarError> {
    Terminal::command()
        .current_dir(path)
        .piped()
        .run("git", vec!["init"])?;
    Ok(())
}
