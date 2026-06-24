use rust_terminal::Terminal;
use solar_core::tool::Plugin::Cargo;

use crate::{
    assert_configuration_file_does_not_exist_at,
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
            ["install", "semverrelease", "--plugins", "cargo"],
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
            ["upgrade", "semverrelease", "--plugins", "cargo"],
        )
        .unwrap();

    // Assert upgraded correctly.
    println!("Checking upgrade...");
    assert_installation(temp.env().path(), Some(vec![Cargo]), true);
    assert_configuration(temp.env().path(), Some(vec![Cargo]));
    println!("Upgrade confirmed!");
}
