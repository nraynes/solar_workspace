use std::{fs, path::Path};

use solar_core::Config;
use toml::Table;

use crate::{assert, assert_eq};

mod double_install;
mod install_no_args;
mod operations_default;
mod uninstall_no_install;
mod uninstall_one;

pub fn assert_configuration(path: &Path, allow_licenses: Vec<&str>, not: bool) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let cargo_deny_config = solar_config.cargo_deny().as_ref().unwrap();
    println!("Checking script in configuration.");
    assert_eq(
        cargo_deny_config.allow_licenses(),
        &Some(allow_licenses.iter().map(|v| v.to_string()).collect()),
        not,
    );
}

pub fn assert_installation(path: &Path, expected_allow_licenses: Vec<&str>, not: bool) {
    println!("Checking toml existence.");
    let cargo_toml_path = path.join("deny.toml");
    assert(fs::exists(&cargo_toml_path).unwrap(), not);
    if !not {
        println!("Checking toml contents.");
        let deny_toml_contents = fs::read_to_string(cargo_toml_path).unwrap();
        let cargo_toml_object = deny_toml_contents
            .parse::<Table>()
            .expect("Could not parse toml file.");
        let mut actual_allow_licenses: Vec<&str> = cargo_toml_object
            .get("licenses")
            .expect("Section 'licenses' not found in toml file.")
            .as_table()
            .ok_or("'licenses' is not formatted correctly.")
            .unwrap()
            .get("allow")
            .expect("Key 'allow' not found in toml file.")
            .as_array()
            .ok_or("Key 'allow' is not a list of values!")
            .unwrap()
            .iter()
            .map(|v| {
                v.as_str()
                    .ok_or("A non-string value was found in list under key 'allow'.")
                    .unwrap()
            })
            .collect();
        actual_allow_licenses.sort();
        assert_eq!(actual_allow_licenses, expected_allow_licenses);
    }
}
