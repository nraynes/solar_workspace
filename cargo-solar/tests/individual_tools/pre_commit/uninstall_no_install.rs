use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{assert_configuration_file_does_not_exist_at, setup_env};

#[test]
pub fn uninstall_no_install() {
    let mut temp = setup_env();

    // Run uninstall
    let uninstall_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["uninstall", "precommit"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(uninstall_output.stderr)
            .unwrap()
            .contains("No such file or directory")
    );

    // Assert uninstalled correctly.
    println!("Checking uninstall...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert!(!fs::exists(temp.env().path().join(".git")).unwrap());
    println!("Uninstall confirmed!");
}
