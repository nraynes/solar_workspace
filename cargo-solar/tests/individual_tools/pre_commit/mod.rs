use std::{fs, path::Path};

use solar_core::{Config, Global, tool::pre_commit::Script};

use crate::assert;

mod install_no_script;
mod operations_default;
mod uninstall_no_install;

pub fn assert_configuration(path: &Path, script: Option<Script>) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let pre_commit_config = solar_config.pre_commit().as_ref().unwrap();
    println!("Checking script in configuration.");
    assert_eq!(pre_commit_config.script(), &script);
}

pub fn assert_installation(path: &Path, script: Option<Script>) {
    println!("Checking git.");
    assert!(fs::exists(path.join(".git")).unwrap());
    println!("Checking script path.");
    let path_to_pre_commit_hook = path
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
    assert(
        fs::exists(&path_to_pre_commit_hook).unwrap(),
        script.is_some(),
    );
    if let Some(script) = script {
        println!("Checking script contents.");
        let script_contents = fs::read_to_string(path_to_pre_commit_hook).unwrap();
        assert_eq!(script_contents, script.content().to_string());
    }
}
