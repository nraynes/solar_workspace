use rust_terminal::Terminal;

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
        .run("./cargo-solar", ["solar", "install", "semverrelease"])
        .unwrap();

    // Assert installed correctly.
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);

    // Run second install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "install", "semverrelease"])
        .unwrap();

    // Assert second install.
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);
}
