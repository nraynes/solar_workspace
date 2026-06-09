use std::{fs, path::Path};

use solar_core::{Config, Global, tool::pre_commit::Script};

use crate::{assert, assert_eq};

mod install_no_script;
mod operations_default;

pub fn assert_configuration(path: &Path, script: Script, not: bool) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let pre_commit_config = solar_config.pre_commit().as_ref().ok_or("").unwrap();
    println!("Checking script in configuration.");
    assert_eq(pre_commit_config.script().as_ref().unwrap(), &script, not);
}

pub fn assert_installation(path: &Path, script: Script, not: bool) {
    println!("Checking git.");
    assert!(fs::exists(path.join(".git")).unwrap());
    println!("Checking script path.");
    let path_to_script = path
        .join(
            Global::git_hooks_path(path)
                .unwrap()
                .to_str()
                .ok_or("Could not convert path to string.")
                .unwrap(),
        )
        .canonicalize()
        .unwrap()
        .join("pre-commit");
    assert(fs::exists(&path_to_script).unwrap(), not);
    if !not {
        println!("Checking script contents.");
        let script_contents = fs::read_to_string(path_to_script).unwrap();
        assert_eq!(script_contents, script.content().to_string());
    }
}
