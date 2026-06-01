use std::{fs, path::PathBuf};

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{
    individual_tools::vhooks::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn operations_default() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run("./cargo-solar", ["install", "vhooks"])
        .unwrap();

    // Assert installed correctly
    println!("Checking installation...");
    assert_installation(temp.env().path().clone(), ".hooks", false);
    assert_configuration(temp.env().path().clone(), ".hooks", false, false);
    println!("Installation confirmed!");

    // Add some hooks
    fs::File::create(temp.env().dir(".hooks").unwrap().path().join("pre-commit")).unwrap();
    fs::File::create(temp.env().dir(".hooks").unwrap().path().join("commit-msg")).unwrap();

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path().clone())
        .run("./cargo-solar", ["upgrade", "vhooks"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Upgrade does not apply to vhooks - nothing to upgrade.")
    );

    // Assert installation doesn't change
    println!("Checking upgrade...");
    assert_installation(temp.env().path().clone(), ".hooks", false);
    assert_configuration(temp.env().path().clone(), ".hooks", false, false);
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run("./cargo-solar", ["uninstall", "vhooks"])
        .unwrap();

    // Assert uninstalled correctly (does not uninstall git)
    println!("Checking uninstall...");
    assert!(!fs::exists(temp.env().path().join(PathBuf::from(SOLARCONFIGNAME))).unwrap());
    assert_installation(temp.env().path().clone(), ".hooks", true);
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
    println!("Uninstall confirmed!");
}
