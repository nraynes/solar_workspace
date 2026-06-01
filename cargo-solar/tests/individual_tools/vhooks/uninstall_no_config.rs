use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::setup_env;

#[test]
pub fn uninstall_no_config() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run("./cargo-solar", ["install", "vhooks"])
        .unwrap();

    // Remove config file
    fs::remove_file(temp.env().path().join(SOLARCONFIGNAME)).unwrap();

    // Run uninstall
    let command_output = Terminal::command()
        .current_dir(temp.env().path().clone())
        .run("./cargo-solar", ["uninstall", "vhooks"])
        .unwrap();

    // Assert that uninstall fails without config
    assert!(
        String::from_utf8(command_output.stderr)
            .unwrap()
            .contains("No such file or directory")
    );
    assert_ne!(command_output.status.code().unwrap(), 0);
}
