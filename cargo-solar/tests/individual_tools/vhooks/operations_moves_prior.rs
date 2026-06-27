use std::fs;

use rust_terminal::Terminal;

use crate::{
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
    println!("Checking installation...");
    assert_installation(temp.env().path(), ".hooks", true);
    assert_configuration(temp.env().path(), ".hooks", true);
    println!("Installation confirmed!");

    // Add some hooks
    fs::File::create(temp.env().dir(".hooks").unwrap().path().join("pre-commit")).unwrap();
    fs::File::create(temp.env().dir(".hooks").unwrap().path().join("commit-msg")).unwrap();

    // Run second install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            ["solar", "install", "vhooks", "--name", "versioned_hooks"],
        )
        .unwrap();

    // Assert installed correctly
    println!("Checking installation...");
    assert_installation(temp.env().path(), "versioned_hooks", true);
    assert_configuration(temp.env().path(), "versioned_hooks", true);

    // Assert hooks moved correctly and old directory deleted
    assert!(!fs::exists(temp.env().path().join(".hooks")).unwrap());
    assert!(
        fs::exists(
            temp.env()
                .dir("versioned_hooks")
                .unwrap()
                .path()
                .join("commit-msg")
        )
        .unwrap()
    );
    assert!(
        fs::exists(
            temp.env()
                .dir("versioned_hooks")
                .unwrap()
                .path()
                .join("pre-commit")
        )
        .unwrap()
    );
    println!("Installation confirmed!");
}
