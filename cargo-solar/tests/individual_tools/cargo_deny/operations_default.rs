use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::cargo_deny::{assert_configuration, assert_installation},
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

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "upgrade", "deny"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installed didn't change.
    println!("Checking upgrade...");
    assert_installation(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));
    assert_configuration(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "deny"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstallation...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None);
    println!("Uninstallation confirmed!");
}
