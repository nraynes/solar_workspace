use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn gitignore(path: &Path) -> Option<Vec<PathBuf>> {
    fs::read_to_string(path.join(".gitignore")).map_or(None, |v| {
        Some(
            v.split('\n')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(PathBuf::from)
                .collect(),
        )
    })
}
