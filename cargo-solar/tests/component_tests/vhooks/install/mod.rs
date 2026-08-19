mod cold;

use std::path::Path;

use affirm_fs::Directory;
use rust_terminal::Terminal;

use solar_core::tools::git::is_git::is_git;

use crate::resources::{CARGO_COMMAND, absolute_git_hooks_path};

pub fn test_vhooks_install(path: &Path, hooks_path: &str) {
    // Get file system snapshot before command runs.
    let snapshot_before = Directory::try_from(path)
        .unwrap()
        .take_and_acquire_contents()
        .unwrap();
    let hooks_dir_contents =
        snapshot_before.dir(&absolute_git_hooks_path(snapshot_before.path()).unwrap());

    // Run command.
    Terminal::command()
        .current_dir(path)
        .piped()
        .run(
            CARGO_COMMAND,
            ["solar", "install", "vhooks", "--hooks-path", hooks_path],
        )
        .unwrap();

    // Get file system snapshot after command runs.
    let snapshot_after = Directory::try_from(path).unwrap();

    // Test that installation matches expected output.
    let hooks_path_in_fs = snapshot_after.path().join(hooks_path);

    assert!(is_git(snapshot_after.path()).unwrap());
    assert!(snapshot_after.dir(&hooks_path_in_fs).is_some());
    assert_eq!(
        &absolute_git_hooks_path(snapshot_after.path())
            .unwrap()
            .canonicalize()
            .unwrap(),
        &hooks_path_in_fs,
    );
    if let Some(original_hooks_dir) = hooks_dir_contents {
        assert!(
            snapshot_after
                .dir(&hooks_path_in_fs)
                .unwrap()
                .deep_eq()
                .dir_weak(original_hooks_dir)
                .unwrap()
        );
    }
}
