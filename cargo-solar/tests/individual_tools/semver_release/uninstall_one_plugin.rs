use rust_terminal::Terminal;
use solar_core::tool::Plugin::Cargo;

use crate::{
    individual_tools::semver_release::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            ["solar", "install", "semverrelease", "--plugins", "cargo"],
        )
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(temp.env().path(), Some(vec![Cargo]), true);
    assert_configuration(temp.env().path(), Some(vec![Cargo]));
    println!("Installation confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            ["solar", "uninstall", "semverrelease", "--plugins", "cargo"],
        )
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstallation...");
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);
    println!("Uninstallation confirmed!");
}
