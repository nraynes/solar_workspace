use std::path::Path;

use rust_terminal::Terminal;

use crate::solar_error::SolarError;

pub fn set_remote_origin(path: &Path, origin: &str) -> Result<(), SolarError> {
    Terminal::command()
        .current_dir(path)
        .piped()
        .run("git", ["remote", "add", "origin", origin])?;
    Ok(())
}
