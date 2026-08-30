use std::{env::current_dir, io, path::PathBuf};

/// Gets the absolute path of the current directory.
pub fn working_dir() -> io::Result<PathBuf> {
    current_dir()?.canonicalize()
}
