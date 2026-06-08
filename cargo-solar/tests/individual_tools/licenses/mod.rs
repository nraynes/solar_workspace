use std::{fs, path::Path};

use solar_core::{Config, LICENSES_DIR};

use crate::{assert, assert_eq};

mod operations_default;

pub fn assert_configuration(
    path: &Path,
    include_licenses: Vec<&str>,
    licensed_under: Vec<&str>,
    not: bool,
) {
    let solar_config = Config::load_from(path).unwrap();
    let licenses_config = solar_config.licenses().as_ref().ok_or("").unwrap();
    assert_eq(
        licenses_config.include_licenses(),
        &Some(include_licenses.iter().map(|s| s.to_string()).collect()),
        not,
    );
    assert_eq(
        licenses_config.licensed_under(),
        &Some(licensed_under.iter().map(|s| s.to_string()).collect()),
        not,
    );
}

pub fn assert_installation(
    path: &Path,
    include_licenses: Vec<&str>,
    licensed_under: Vec<&str>,
    not: bool,
) {
    assert(fs::exists(path.join(LICENSES_DIR)).unwrap(), not);
    for license in include_licenses {
        assert(
            fs::exists(path.join(LICENSES_DIR).join(license)).unwrap(),
            not,
        );
    }
    for license in licensed_under {
        assert(fs::exists(path.join(license)).unwrap(), not);
    }
}
