use std::{env::current_dir, fs, path::Path};

pub fn copy_bin(path: &Path) {
    let mut workspace = current_dir().unwrap();
    workspace.pop();
    fs::copy(
        workspace.join("target/debug/cargo-solar"),
        path.join("cargo-solar"),
    )
    .unwrap();
}
