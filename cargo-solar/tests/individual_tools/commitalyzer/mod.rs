use std::{fs, path::Path};

use solar_core::Config;

use crate::{assert, assert_eq};

mod double_install;
mod install_no_args;
mod operations_default;
mod uninstall_no_install;
mod upgrade_no_args;
mod upgrade_no_install;

pub static RULESET_FOR_TESTING: &str = "conventional-commits";

pub fn assert_configuration(path: &Path, ruleset: &Option<String>, not: bool) {
    println!("Getting configuration.");
    let solar_config = Config::load_from(path).unwrap();
    let commitalyzer_config = solar_config.commitalyzer().as_ref().unwrap();
    println!("Checking for rules list configuration.");
    assert_eq(commitalyzer_config.ruleset(), ruleset, not);
}

pub fn assert_installation(path: &Path, ruleset: &Option<String>, not: bool, git_not: bool) {
    if !git_not {
        println!("Checking for git.");
        assert!(fs::exists(path.join(".git")).unwrap());
    }
    println!("Checking for commit-msg hook.");
    assert(fs::exists(path.join(".git/hooks/commit-msg")).unwrap(), not);
    println!("Checking for commit-rules directory.");
    assert(fs::exists(path.join("commit-rules")).unwrap(), not);
    if !not {
        let ruleset = ruleset.clone().expect(
            "A ruleset name must given to check for a ruleset. This is an error with the test.",
        );
        println!(
            "Checking for {}.yml file in commit-rules directory.",
            ruleset
        );
        assert!(fs::exists(path.join(format!("commit-rules/{}.yml", ruleset))).unwrap());
    }
}
