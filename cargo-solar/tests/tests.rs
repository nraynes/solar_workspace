mod individual_tools;
mod project_configs;

use std::{env::current_dir, fs, path::PathBuf};

use mocked_up::TempEnv;

pub fn copy_bin(path: &PathBuf) {
    let mut workspace = current_dir().unwrap();
    workspace.pop();
    fs::copy(
        workspace.join(PathBuf::from("target/debug/cargo-solar")),
        path.join("cargo-solar"),
    )
    .unwrap();
}

pub fn setup_env() -> TempEnv {
    let mut temp = TempEnv::new().unwrap();
    copy_bin(temp.env().path());
    temp
}
