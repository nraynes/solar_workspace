use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{
    individual_tools::cargo_deny::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn operations_default() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            [
                "install",
                "deny",
                "--allow-licenses",
                "MIT-1.0",
                "Unicode-3.0",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], false);
    assert_configuration(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], false);
    println!("Installation confirmed!");

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["upgrade", "deny"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installed didn't change.
    println!("Checking upgrade...");
    assert_installation(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], false);
    assert_configuration(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], false);
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["uninstall", "deny"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstallation...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    assert_installation(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], true);
    println!("Uninstallation confirmed!");
}
