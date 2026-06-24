use std::{fs, path::Path, str::FromStr};

use serde_json::Value;
use solar_core::{
    Config,
    tool::{Plugin, RELEASE_BIN_NAME, RELEASE_CONFIG_NAME, RELEASE_DIR_NAME},
};

mod double_install;
mod operations_duplicate_plugin;
mod operations_no_plugins;
mod operations_one_plugin;
mod uninstall_no_install;
mod uninstall_one_plugin;
mod upgrade_no_install;
mod upgrade_one_plugin;

use crate::{assert, assert_opt_vec_eq_unord};

pub fn assert_configuration(path: &Path, expected_plugins: Option<Vec<Plugin>>) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let semver_release_config = solar_config.semver_release().as_ref().unwrap();
    println!("Checking plugins configuration.");
    let actual_plugins = semver_release_config.plugins();
    assert_opt_vec_eq_unord(actual_plugins, &expected_plugins, true);
}

pub fn assert_installation(path: &Path, expected_plugins: Option<Vec<Plugin>>, assert_true: bool) {
    println!("Checking semver-release bin existence.");
    assert(
        fs::exists(path.join(RELEASE_DIR_NAME)).unwrap(),
        assert_true,
    );
    assert(
        fs::exists(path.join(RELEASE_DIR_NAME).join(RELEASE_BIN_NAME)).unwrap(),
        assert_true,
    );

    println!("Checking configuration file.");
    assert(
        fs::exists(path.join(RELEASE_CONFIG_NAME)).unwrap(),
        assert_true,
    );
    if assert_true {
        let config_contents = Value::from_str(
            &fs::read_to_string(path.join(RELEASE_CONFIG_NAME))
                .expect("No configuration file found."),
        )
        .unwrap();
        let plugin_configs = config_contents
            .as_object()
            .ok_or("Could not parse release config.")
            .unwrap()
            .get("plugins")
            .ok_or("Could not extract plugins config.")
            .unwrap()
            .as_object()
            .ok_or("Could not parse plugins config.")
            .unwrap();
        if let Some(plugins) = expected_plugins {
            println!("Checking plugins.");
            for plugin in plugins {
                println!("Checking plugin {} bin existence.", plugin.bin_name());
                assert(
                    fs::exists(path.join(RELEASE_DIR_NAME).join(plugin.bin_name())).unwrap(),
                    assert_true,
                );

                println!("Checking plugin {} config existence.", plugin.bin_name());
                let plugin_config_result = plugin_configs
                    .get(plugin.bin_name())
                    .ok_or(format!("Plugin {}:", plugin.bin_name()));
                match assert_true {
                    true => plugin_config_result.expect("Configuration not found."),
                    false => plugin_config_result
                        .expect("Configuration found when it should not have been."),
                };
            }
        }
    }
}
