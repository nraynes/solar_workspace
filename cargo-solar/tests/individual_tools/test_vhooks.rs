use std::{env::current_dir, fs, path::PathBuf};

use mocked_up::TempEnv;
use rust_terminal::Terminal;
use solar_core::{Config, SOLARCONFIGNAME};

fn git_hooks_path(path: PathBuf) -> String {
    let command_output = Terminal::command()
        .current_dir(path)
        .run("git", ["config", "core.hooksPath"])
        .unwrap();
    String::from_utf8(command_output.stdout).unwrap()
}

fn assert(input: bool, not: bool) {
    assert!(match not {
        true => !input,
        false => input,
    });
}

fn assert_eq<T>(x: T, y: T, not: bool)
where
    T: PartialEq + std::fmt::Debug,
{
    match not {
        true => assert_ne!(x, y),
        false => assert_eq!(x, y),
    }
}

fn assert_configuration(path: PathBuf, name: &str, remove_all: bool, not: bool) {
    let solar_config = Config::load_from_file(path.join(PathBuf::from(SOLARCONFIGNAME))).unwrap();
    let vhooks_config = solar_config.vhooks().as_ref().ok_or("").unwrap();
    assert_eq(vhooks_config.name(), &name.to_string(), not);
    assert_eq(vhooks_config.remove_all(), &remove_all, not);
}

fn assert_installation(path: PathBuf, name: &str, not: bool) {
    assert!(fs::exists(path.join(".git")).unwrap());
    assert(fs::exists(path.join(name)).unwrap(), not);
    assert_eq(git_hooks_path(path), format!("./{}\n", name), not);
}

fn copy_bin(path: &PathBuf) {
    let mut workspace = current_dir().unwrap();
    workspace.pop();
    fs::copy(
        workspace.join(PathBuf::from("target/debug/cargo-solar")),
        path.join("cargo-solar"),
    )
    .unwrap();
}

fn setup_env() -> TempEnv {
    let mut temp = TempEnv::new().unwrap();
    copy_bin(temp.env().path());
    temp
}

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

#[test]
pub fn uninstall_no_config() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run("./cargo-solar", ["install", "vhooks"])
        .unwrap();

    // Remove config file
    fs::remove_file(temp.env().path().join(SOLARCONFIGNAME)).unwrap();

    // Run uninstall
    let command_output = Terminal::command()
        .current_dir(temp.env().path().clone())
        .run("./cargo-solar", ["uninstall", "vhooks"])
        .unwrap();

    // Assert that uninstall fails without config
    assert!(
        String::from_utf8(command_output.stderr)
            .unwrap()
            .contains("No such file or directory")
    );
    assert_ne!(command_output.status.code().unwrap(), 0);
}

#[test]
pub fn uninstall_no_vhooks() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path().clone())
        .piped()
        .run("./cargo-solar", ["install", "vhooks"])
        .unwrap();

    // Remove vhooks from config file
    fs::remove_file(temp.env().path().join(SOLARCONFIGNAME)).unwrap();
    Config::new(None, None, None, None, None, None, None)
        .save_to_file(temp.env().path().join(SOLARCONFIGNAME))
        .unwrap();

    // Run uninstall
    let command_output = Terminal::command()
        .current_dir(temp.env().path().clone())
        .run("./cargo-solar", ["uninstall", "vhooks"])
        .unwrap();

    // Assert that uninstall fails without config
    assert!(
        String::from_utf8(command_output.stderr)
            .unwrap()
            .contains("Cannot uninstall vhooks - vhooks not found in configuration.")
    );
    assert_ne!(command_output.status.code().unwrap(), 0);
}
