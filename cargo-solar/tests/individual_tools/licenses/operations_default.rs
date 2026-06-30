use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::licenses::{assert_configuration, assert_installation},
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
                "licenses",
                "--include-licenses",
                "MIT",
                "Apache-2.0",
                "--licensed-under",
                "MIT",
                "Apache-2.0",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    assert_installation(
        temp.env().path(),
        Some(vec!["LICENSE-MIT", "LICENSE-Apache-2.0"]),
        Some(vec!["LICENSE-MIT", "LICENSE-Apache-2.0"]),
        true,
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0"]),
        Some(vec!["MIT", "Apache-2.0"]),
    );

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "upgrade", "licenses"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installed doesn't change.
    assert_installation(
        temp.env().path(),
        Some(vec!["LICENSE-MIT", "LICENSE-Apache-2.0"]),
        Some(vec!["LICENSE-MIT", "LICENSE-Apache-2.0"]),
        true,
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0"]),
        Some(vec!["MIT", "Apache-2.0"]),
    );

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "licenses"])
        .unwrap();

    // Assert uninstalled correctly.
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, None, false, false);
}
