use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::licenses::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn operations_default() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            [
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
    println!("Checking installation...");
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
    println!("Installation confirmed!");

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["upgrade", "licenses"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installed doesn't change.
    println!("Checking upgrade...");
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
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["uninstall", "licenses"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstall...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, None, false, false);
    println!("Uninstall confirmed!");
}
