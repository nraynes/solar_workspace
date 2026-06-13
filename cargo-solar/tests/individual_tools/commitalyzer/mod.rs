
use std::path::Path;

use solar_core::Config;

use crate::{assert, assert_eq};

pub fn assert_configuration(path: &Path, not: bool) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let cargo_deny_config = solar_config.cargo_deny().as_ref().ok_or("").unwrap();
}

pub fn assert_installation(path: &Path, not: bool) {

}
