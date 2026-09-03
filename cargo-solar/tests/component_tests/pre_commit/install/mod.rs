use std::path::Path;

use affirm_fs::contains_subslice;
use clap::ValueEnum;
use rust_terminal::Terminal;
use solar_core::components::pre_commit::{PRECOMMIT_ALREADY_EXISTS_ERR_MSG, Script};

use crate::{component_tests::pre_commit::Snapshot, resources::CARGO_COMMAND};

mod cold_hot;
mod no_git;

pub fn test_install_was_successful(path: &Path, script_name: &str, force_overwrite: bool) {
    let script = Script::from_str(script_name, true).unwrap();

    let mut command_args = vec!["solar", "install", "pre-commit", "--script", script_name];

    if force_overwrite {
        command_args.push("-f");
    }

    // Get file system snapshot before command runs.
    let snapshot_before = Snapshot::from(path);

    // Run command.
    let stderr = Terminal::command()
        .current_dir(path)
        .piped()
        .stderr(CARGO_COMMAND, command_args)
        .unwrap();

    // Get file system snapshot after command runs.
    let snapshot_after = Snapshot::from(path);

    assert!(snapshot_after.is_git());

    if let Some(pre_commit_file_before) = snapshot_before.pre_commit_file()
        && !force_overwrite
    {
        assert!(contains_subslice(
            stderr.as_bytes(),
            PRECOMMIT_ALREADY_EXISTS_ERR_MSG.as_bytes()
        ));
        assert!(
            snapshot_after
                .pre_commit_file()
                .as_ref()
                .is_some_and(|pre_commit_file_after| pre_commit_file_after
                    .eq()
                    .file(pre_commit_file_before))
        )
    } else {
        let pre_commit_file_after = snapshot_after.pre_commit_file().as_ref().unwrap();
        let pre_commit_contents = pre_commit_file_after.static_content.as_ref().unwrap();
        assert_eq!(pre_commit_contents, script.content().as_bytes());
    }
}
