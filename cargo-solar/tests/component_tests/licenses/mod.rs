use std::{fs, path::Path};

use solar_core::{Config, LICENSES_DIR};

use crate::{assert, assert_opt_vec_eq_unord};

mod double_install;
mod install_no_args;
mod operations_default;
mod operations_with_args;
mod uninstall_no_install;
mod uninstall_one;

pub fn assert_configuration(
    path: &Path,
    include_licenses: Option<Vec<&str>>,
    licensed_under: Option<Vec<&str>>,
) {
    let solar_config = Config::load_from(path).unwrap();
    let licenses_config = solar_config.licenses().as_ref().unwrap();

    assert_opt_vec_eq_unord(
        &include_licenses.map_or(None, |c| Some(c.iter().map(|s| s.to_string()).collect())),
        licenses_config.include_licenses(),
        true,
    );
    assert_opt_vec_eq_unord(
        &licensed_under.map_or(None, |c| Some(c.iter().map(|s| s.to_string()).collect())),
        licenses_config.licensed_under(),
        true,
    );
}

pub fn assert_installation(
    path: &Path,
    include_licenses: Option<Vec<&str>>,
    licensed_under: Option<Vec<&str>>,
    include_licenses_should_exist: bool,
    licensed_under_should_exist: bool,
) {
    assert(
        fs::exists(path.join(LICENSES_DIR)).unwrap(),
        include_licenses.is_some(),
    );
    if let Some(licenses) = include_licenses {
        for license in licenses {
            println!(
                "Checking if {} in license directory is in existence.",
                license
            );
            assert(
                fs::exists(path.join(LICENSES_DIR).join(license)).unwrap(),
                include_licenses_should_exist,
            );
        }
    }
    if let Some(licenses) = licensed_under {
        for license in licenses {
            assert(
                fs::exists(path.join(license)).unwrap(),
                licensed_under_should_exist,
            );
        }
    }
}
