use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{
    individual_tools::cargo_deny::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn uninstall_one() {
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

    // Run uninstall for one argument.
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            ["uninstall", "deny", "--allow-licenses", "MIT-1.0"],
        )
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstallation...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    assert_installation(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], true);
    println!("Uninstallation confirmed!");
}
