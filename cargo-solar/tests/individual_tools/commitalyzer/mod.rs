use std::{fs, path::Path};

use solar_core::{Config, components::Ruleset};

use crate::{assert, assert_eq};

mod double_install;
mod install_no_args;
mod operations_default;
mod uninstall_no_install;
mod upgrade_no_install;
mod upgrade_with_args;

pub static RULESET_FOR_TESTING: Ruleset = Ruleset::ConventionalCommits;

pub fn assert_configuration(path: &Path, ruleset: &Option<Ruleset>) {
    let solar_config = Config::load_from(path).unwrap();
    let commitalyzer_config = solar_config.commitalyzer().as_ref().unwrap();
    assert_eq(commitalyzer_config.ruleset(), ruleset, true);
}

pub fn assert_installation(
    path: &Path,
    ruleset: &Option<Ruleset>,
    commit_msg_hook_should_exist: bool,
    ruleset_dir_should_exist: bool,
    ruleset_should_exist: bool,
    git_should_exist: bool,
) {
    if git_should_exist {
        assert!(fs::exists(path.join(".git")).unwrap());
    }
    assert(
        fs::exists(path.join(".git/hooks/commit-msg")).unwrap(),
        commit_msg_hook_should_exist,
    );
    assert(
        fs::exists(path.join("commit-rules")).unwrap(),
        ruleset_dir_should_exist,
    );
    if ruleset_should_exist {
        let ruleset = ruleset.clone().expect(
            "A ruleset name must given to check for a ruleset. This is an error with the test.",
        );
        println!(
            "Checking for {}.yml file in commit-rules directory.",
            ruleset.get()
        );
        assert!(fs::exists(path.join(format!("commit-rules/{}.yml", ruleset.get()))).unwrap());
    }
}
