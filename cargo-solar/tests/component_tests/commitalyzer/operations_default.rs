use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::commitalyzer::{
        RULESET_FOR_TESTING, assert_configuration, assert_installation,
    },
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
            [
                "solar",
                "install",
                "commitalyzer",
                "--ruleset",
                "conventional-commits",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING),
        true,
        true,
        true,
        true,
    );
    assert_configuration(temp.env().path(), &Some(RULESET_FOR_TESTING));

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "upgrade", "commitalyzer"])
        .unwrap();

    // Assert no error.
    assert_eq!(upgrade_output.status.code(), Some(0));

    // Assert upgraded correctly.
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING),
        true,
        true,
        true,
        true,
    );
    assert_configuration(temp.env().path(), &Some(RULESET_FOR_TESTING));

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "commitalyzer"])
        .unwrap();

    // Assert uninstalled correctly.
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING),
        false,
        false,
        false,
        true,
    );
}
