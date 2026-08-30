use std::{fs, path::Path};

use crate::solar_error::SolarError;

pub fn move_hooks(prev: &Path, new: &Path) -> Result<(), SolarError> {
    for item in fs::read_dir(prev)? {
        let item = item?;
        fs::rename(item.path(), new.join(item.file_name()))?;
    }
    Ok(())
}
