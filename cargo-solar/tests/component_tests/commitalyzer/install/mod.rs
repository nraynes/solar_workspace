use std::path::Path;

use rust_terminal::Terminal;

use crate::{component_tests::commitalyzer::Snapshot, resources::CARGO_COMMAND};

mod cold_hot;
mod no_git;

pub fn test_install_was_successful(path: &Path, commit_rules: &[&str]) {
    let commit_rules = commit_rules.to_vec();
    let mut command_args = vec!["solar", "install", "commitalyzer"];

    if !commit_rules.is_empty() {
        command_args.push("--rulesets");
        for ruleset in &commit_rules {
            command_args.push(ruleset);
        }
    }

    Terminal::command()
        .current_dir(path)
        .piped()
        .run(CARGO_COMMAND, command_args)
        .unwrap();

    let snapshot_after = Snapshot::from(path);
    assert!(snapshot_after.is_git());
    assert!(snapshot_after.commit_msg_hook().is_some());
    let commit_rules_dir = snapshot_after.commit_rules().as_ref().unwrap();

    if commit_rules.is_empty() {
        assert!(
            commit_rules_dir
                .contains()
                .file_named("conventional-commits.yml")
        );
    } else {
        for ruleset in commit_rules {
            assert!(
                commit_rules_dir
                    .contains()
                    .file_named(&format!("{}.yml", ruleset))
            );
        }
    }
}
