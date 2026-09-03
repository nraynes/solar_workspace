mod install;
mod uninstall;

use std::path::Path;

use derive_getters::Getters;
use solar_core::{
    components::cargo_deny::{CARGO_DENY_CRATE_NAME, DENY_TOML_NAME},
    tools::cargo::is_crate_installed::is_crate_installed,
};

#[derive(Getters, Debug)]
pub struct Snapshot {
    crate_installed: bool,
    deny_toml: Option<toml::Value>,
}

impl From<&Path> for Snapshot {
    fn from(value: &Path) -> Self {
        Self {
            crate_installed: is_crate_installed(CARGO_DENY_CRATE_NAME).unwrap(),
            deny_toml: affirm_fs::File::try_from(value.join(DENY_TOML_NAME))
                .ok()
                .and_then(|f| {
                    toml::from_str(String::from_utf8(f.contents().unwrap()).unwrap().as_str())
                        .unwrap()
                }),
        }
    }
}
