use std::fs;

use rust_terminal::Terminal;
use solar_core::{SOLARCONFIGNAME, tool::pre_commit::Script};

use crate::{
    individual_tools::pre_commit::{assert_configuration, assert_installation},
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
            ["install", "precommit", "--script", "cargo-basic"],
        )
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(temp.env().path(), Script::CargoBasic, false);
    assert_configuration(temp.env().path(), Script::CargoBasic, false);
    println!("Installation confirmed!");

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["upgrade", "precommit"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installed doesn't change.
    println!("Checking upgrade...");
    assert_installation(temp.env().path(), Script::CargoBasic, false);
    assert_configuration(temp.env().path(), Script::CargoBasic, false);
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["uninstall", "precommit"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstall...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    assert_installation(temp.env().path(), Script::CargoBasic, true);
    println!("Uninstall confirmed!");
}
