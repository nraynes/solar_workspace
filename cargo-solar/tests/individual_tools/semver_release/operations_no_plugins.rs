use rust_terminal::Terminal;

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
        .run("./cargo-solar", ["install", "semverrelease"])
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);
    println!("Installation confirmed!");

    // Run upgrade
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["upgrade", "semverrelease"])
        .unwrap();

    // Assert upgraded correctly.
    println!("Checking upgrade...");
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["uninstall", "semverrelease"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstallation...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, false);
    println!("Uninstallation confirmed!");
}
