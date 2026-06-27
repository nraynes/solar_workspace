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
    println!("Checking installation...");
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING.to_string()),
        true,
        true,
        true,
        true,
    );
    assert_configuration(temp.env().path(), &Some(RULESET_FOR_TESTING.to_string()));
    println!("Installation confirmed!");

    // Run upgrade
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            [
                "solar",
                "upgrade",
                "commitalyzer",
                "--ruleset",
                "conventional-commits",
            ],
        )
        .unwrap();

    // Assert upgraded correctly.
    println!("Checking upgrade...");
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING.to_string()),
        true,
        true,
        true,
        true,
    );
    assert_configuration(temp.env().path(), &Some(RULESET_FOR_TESTING.to_string()));
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "commitalyzer"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstallation...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING.to_string()),
        false,
        false,
        false,
        true,
    );
    println!("Uninstallation confirmed!");
}
