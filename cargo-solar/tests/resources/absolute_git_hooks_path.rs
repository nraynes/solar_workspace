use std::path::{Path, PathBuf};

use solar_core::{solar_error::SolarError, tools::git::git_hooks_path};

pub fn absolute_git_hooks_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, SolarError> {
    let path = path.as_ref();
    let git_hooks_path = git_hooks_path(path)?;
    Ok(match git_hooks_path.is_relative() {
        true => path.join(git_hooks_path),
        false => git_hooks_path,
    })
}
