use std::fs;

use rust_terminal::Terminal;

use crate::{individual_tools::vhooks::assert_installation, setup_env};

#[test]
pub fn operations_with_arguments() {
    let mut temp = setup_env();
    let proj_name = "proj_folder";
    temp.env().mkdir(proj_name).unwrap();
    let proj_path = temp
        .env()
        .dir(proj_name)
        .ok_or("No project found.")
        .unwrap()
        .path()
        .clone();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run(
            "./cargo-solar",
            [
                "install",
                "vhooks",
                "--destination",
                proj_path
                    .to_str()
                    .ok_or("Could not convert project folder path to string")
                    .unwrap(),
                "--name",
                "versioned_hooks",
                "--remove-all",
            ],
        )
        .unwrap();

    // Assert installed correctly
    println!("Checking installation...");
    assert_installation(proj_path.clone(), "versioned_hooks", false);
    println!("Installation confirmed!");

    // Add some hooks
    fs::File::create(
        temp.env()
            .dir("proj_folder")
            .unwrap()
            .dir("versioned_hooks")
            .unwrap()
            .path()
            .join("pre-commit"),
    )
    .unwrap();
    fs::File::create(
        temp.env()
            .dir("proj_folder")
            .unwrap()
            .dir("versioned_hooks")
            .unwrap()
            .path()
            .join("commit-msg"),
    )
    .unwrap();

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path().clone())
        .run(
            "./cargo-solar",
            [
                "upgrade",
                "vhooks",
                "--destination",
                proj_path
                    .to_str()
                    .ok_or("Could not convert project folder path to string")
                    .unwrap(),
                "--name",
                "versioned_hooks",
                "--remove-all",
            ],
        )
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Upgrade does not apply to vhooks - nothing to upgrade.")
    );

    // Assert installation doesn't change
    println!("Checking upgrade...");
    assert_installation(proj_path.clone(), "versioned_hooks", false);
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run(
            "./cargo-solar",
            [
                "uninstall",
                "vhooks",
                "--destination",
                proj_path
                    .to_str()
                    .ok_or("Could not convert project folder path to string")
                    .unwrap(),
                "--remove-all",
            ],
        )
        .unwrap();

    // Assert uninstalled correctly (does not uninstall git)
    println!("Checking uninstall...");
    assert_installation(proj_path, "versioned_hooks", true);
    assert!(
        !fs::exists(
            temp.env()
                .dir("proj_folder")
                .unwrap()
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
        !fs::exists(
            temp.env()
                .dir("proj_folder")
                .unwrap()
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
