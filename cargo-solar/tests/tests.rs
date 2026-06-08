mod individual_tools;
mod project_configs;

use std::{env::current_dir, fs, path::Path};

use mocked_up::TempEnv;
use rust_terminal::Terminal;

pub fn copy_bin(path: &Path) {
    let mut workspace = current_dir().unwrap();
    workspace.pop();
    fs::copy(
        workspace.join("target/debug/cargo-solar"),
        path.join("cargo-solar"),
    )
    .unwrap();
}

pub fn setup_env() -> TempEnv {
    let mut temp = TempEnv::new().unwrap();
    copy_bin(temp.env().path());
    temp
}

pub fn assert(input: bool, not: bool) {
    assert!(match not {
        true => !input,
        false => input,
    });
}

pub fn assert_eq<T>(x: T, y: T, not: bool)
where
    T: PartialEq + std::fmt::Debug,
{
    match not {
        true => assert_ne!(x, y),
        false => assert_eq!(x, y),
    }
}

pub fn git_hooks_path(path: &Path) -> String {
    let command_output = Terminal::command()
        .current_dir(path)
        .run("git", ["config", "core.hooksPath"])
        .unwrap();
    String::from_utf8(command_output.stdout).unwrap()
}
