use std::{env::current_dir, path::PathBuf};

use crate::solar_error::SolarError;

pub fn working_dir() -> Result<PathBuf, SolarError> {
    Ok(current_dir()?.canonicalize()?)
}
