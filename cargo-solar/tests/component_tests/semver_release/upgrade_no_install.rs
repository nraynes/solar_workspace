use rust_terminal::Terminal;

use crate::{
    individual_tools::{semver_release::assert_configuration, semver_release::assert_installation},
    setup_env,
};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "upgrade", "semverrelease"])
        .unwrap();

    // Assert installed correctly.
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);
}
