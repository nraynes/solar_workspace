use std::{fs, path::Path};

use crate::solar_error::SolarError;

/// Returns whether the path is a git repository or not.
pub fn is_git(path: &Path) -> Result<bool, SolarError> {
    Ok(fs::exists(path.join(".git"))?)
}
