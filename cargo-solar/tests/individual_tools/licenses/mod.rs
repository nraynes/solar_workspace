use std::{fs, path::Path};

use solar_core::{Config, LICENSES_DIR};

use crate::{assert, assert_eq};

mod double_install;
mod operations_default;
mod operations_with_args;
mod uninstall_no_install;
mod uninstall_one;

pub fn assert_configuration(
    path: &Path,
    include_licenses: Vec<&str>,
    licensed_under: Vec<&str>,
    not: bool,
) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let licenses_config = solar_config.licenses().as_ref().ok_or("").unwrap();
    let config_include_licenses: &mut Option<Vec<String>> =
        &mut licenses_config.include_licenses().clone();
    if let Some(l) = config_include_licenses {
        l.sort();
    }
    let actual_include_licenses: &mut Vec<String> =
        &mut include_licenses.iter().map(|s| s.to_string()).collect();
    actual_include_licenses.sort();
    let config_licensed_under: &mut Option<Vec<String>> =
        &mut licenses_config.licensed_under().clone();
    if let Some(l) = config_licensed_under {
        l.sort();
    }
    let actual_licensed_under: &mut Vec<String> =
        &mut licensed_under.iter().map(|s| s.to_string()).collect();
    actual_licensed_under.sort();
    println!("Checking include_licenses is correct.");
    assert_eq(
        config_include_licenses,
        &mut Some(actual_include_licenses.clone()),
        not,
    );
    println!("Checking licensed_under is correct.");
    assert_eq(
        config_licensed_under,
        &mut Some(actual_licensed_under.clone()),
        not,
    );
}

pub fn assert_installation(
    path: &Path,
    include_licenses: Vec<&str>,
    licensed_under: Vec<&str>,
    not: bool,
    license_directory_not: bool,
) {
    println!("Checking license directory existence.");
    assert(
        fs::exists(path.join(LICENSES_DIR)).unwrap(),
        license_directory_not,
    );
    for license in include_licenses {
        println!(
            "Checking if {} in license directory is in existence.",
            license
        );
        assert(
            fs::exists(path.join(LICENSES_DIR).join(license)).unwrap(),
            not,
        );
    }
    for license in licensed_under {
        println!("Checking if license file {} is in existence.", license);
        assert(fs::exists(path.join(license)).unwrap(), not);
    }
}
