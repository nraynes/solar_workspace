use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{individual_tools::cargo_deny::assert_installation, setup_env};

#[test]
pub fn uninstall_no_install() {
    let mut temp = setup_env();

    // Run uninstall
    let uninstall_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["uninstall", "deny"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(uninstall_output.stderr)
            .unwrap()
            .contains("No such file or directory")
    );

    // Assert environment doesn't change.
    println!("Checking no installation...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    assert_installation(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], true);
    println!("No installation confirmed!");
}
