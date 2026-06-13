use std::path::Path;

use solar_core::Config;

use crate::{assert, assert_eq};

pub fn assert_configuration(path: &Path, not: bool) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let semver_release_config = solar_config.semver_release().as_ref().unwrap();
}

pub fn assert_installation(path: &Path, not: bool) {}
