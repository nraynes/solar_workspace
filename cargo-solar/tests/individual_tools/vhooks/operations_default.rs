use std::fs;

use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::vhooks::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "install", "vhooks"])
        .unwrap();

    // Assert installed correctly
    assert_installation(temp.env().path(), ".hooks", true);
    assert_configuration(temp.env().path(), ".hooks", true);

    // Add some hooks
    fs::File::create(temp.env().dir(".hooks").unwrap().path().join("pre-commit")).unwrap();
    fs::File::create(temp.env().dir(".hooks").unwrap().path().join("commit-msg")).unwrap();

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "upgrade", "vhooks"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installation doesn't change
    assert_installation(temp.env().path(), ".hooks", true);
    assert_configuration(temp.env().path(), ".hooks", true);

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "vhooks"])
        .unwrap();

    // Assert uninstalled correctly (does not uninstall git)
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), ".hooks", false);
    assert!(
        fs::exists(
            temp.env()
                .dir(".git")
                .unwrap()
                .dir("hooks")
                .unwrap()
                .path()
                .join("commit-msg")
        )
        .unwrap()
    );
    assert!(
        fs::exists(
            temp.env()
                .dir(".git")
                .unwrap()
                .dir("hooks")
                .unwrap()
                .path()
                .join("pre-commit")
        )
        .unwrap()
    );
}
