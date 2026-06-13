use solar_core::{Config, Global};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{assert, assert_eq};

mod operations_default;
mod operations_moves_prior;
mod operations_with_arguments;
mod uninstall_no_config;
mod uninstall_no_vhooks;

pub fn assert_configuration(path: &Path, name: &str, remove_all: bool, not: bool) {
    let solar_config = Config::load_from(path).unwrap();
    let vhooks_config = solar_config.vhooks().as_ref().unwrap();
    assert_eq(vhooks_config.name(), &name.to_string(), not);
    assert_eq(vhooks_config.remove_all(), &remove_all, not);
}

pub fn assert_installation(path: &Path, name: &str, not: bool) {
    assert!(fs::exists(path.join(".git")).unwrap());
    assert(fs::exists(path.join(name)).unwrap(), not);
    assert_eq(
        Global::git_hooks_path(path).unwrap(),
        PathBuf::from(format!("./{}", name)),
        not,
    );
}
