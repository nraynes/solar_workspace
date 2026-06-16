use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{
    assert_configuration_file_does_not_exist_at,
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
    assert_installation(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));
    assert_configuration(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));
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
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None);
    println!("Uninstallation confirmed!");
}
