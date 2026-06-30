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
        .run("./cargo-solar", ["solar", "install", "semverrelease"])
        .unwrap();

    // Assert installed correctly.
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);

    // Run upgrade
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "upgrade", "semverrelease"])
        .unwrap();

    // Assert upgraded correctly.
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "semverrelease"])
        .unwrap();

    // Assert uninstalled correctly.
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, false);
}
