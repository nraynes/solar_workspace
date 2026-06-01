use std::fs;

use rust_terminal::Terminal;
use solar_core::{Config, SOLARCONFIGNAME};

use crate::setup_env;

#[test]
pub fn uninstall_no_vhooks() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run("./cargo-solar", ["install", "vhooks"])
        .unwrap();

    // Remove vhooks from config file
    fs::remove_file(temp.env().path().join(SOLARCONFIGNAME)).unwrap();
    Config::new_empty()
        .save_to_file(temp.env().path().join(SOLARCONFIGNAME))
        .unwrap();

    // Run uninstall
    let command_output = Terminal::command()
        .current_dir(temp.env().path().clone())
        .run("./cargo-solar", ["uninstall", "vhooks"])
        .unwrap();

    // Assert that uninstall fails without config
    assert!(
        String::from_utf8(command_output.stderr)
            .unwrap()
            .contains("Cannot uninstall vhooks - vhooks not found in configuration.")
    );
    assert_ne!(command_output.status.code().unwrap(), 0);
}
