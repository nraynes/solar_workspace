use std::{fs, path::Path};

use solar_core::Config;
use toml::Table;

use crate::{assert, assert_opt_vec_eq_unord, assert_vec_eq_unord};

mod double_install;
mod install_no_args;
mod operations_default;
mod uninstall_no_install;
mod uninstall_one;

pub fn assert_configuration(path: &Path, allow_licenses: Option<Vec<&str>>) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let cargo_deny_config = solar_config.cargo_deny().as_ref().unwrap();
    println!("Checking script in configuration.");
    assert_opt_vec_eq_unord(
        cargo_deny_config.allow_licenses(),
        &allow_licenses.map_or(None, |x| Some(x.iter().map(|y| y.to_string()).collect())),
        true,
    );
}

pub fn assert_installation(path: &Path, allow_licenses: Option<Vec<&str>>) {
    println!("Checking toml existence.");
    let cargo_toml_path = path.join("deny.toml");
    assert(
        fs::exists(&cargo_toml_path).unwrap(),
        allow_licenses.is_some(),
    );
    if let Some(expected_allow_licenses) = allow_licenses {
        println!("Checking toml contents.");

        // Extract deny.toml configuration.
        let deny_toml_contents = fs::read_to_string(cargo_toml_path).unwrap();
        let deny_toml_map = deny_toml_contents
            .parse::<Table>()
            .expect("Could not parse toml file.");

        // Extract value from configuration at 'allow' key under 'licenses' section.
        let actual_allow_licenses: Vec<&str> = deny_toml_map
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

        // Assert equality of expected versus actual allowed licenses.
        assert_vec_eq_unord(&actual_allow_licenses, &expected_allow_licenses, true);
    }
}
