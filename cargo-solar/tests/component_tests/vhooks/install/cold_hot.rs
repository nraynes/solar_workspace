use std::path::Path;

use rust_terminal::Terminal;

use crate::{
    component_tests::vhooks::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

fn test_install_was_successful(path: &Path, hooks_path: &str) {
    // Get file system snapshot before command runs.
    let snapshot_before = Snapshot::from(path);

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
    let snapshot_after = Snapshot::from(path);

    assert!(snapshot_after.is_git());

    let hooks_dir_after = snapshot_after.hooks_dir().as_ref().unwrap();

    assert_eq!(hooks_dir_after.path(), &path.join(hooks_path));

    if let Some(hooks_dir_before) = snapshot_before.hooks_dir() {
        assert!(
            hooks_dir_after
                .deep_eq()
                .dir_weak(hooks_dir_before)
                .unwrap()
        );
    }
}

#[test]
pub fn vhooks() {
    let mut temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    // Test cold install (install with no prior install).
    test_install_was_successful(temp.env().path(), ".hooks");

    // Test hot install (install with prior install).
    test_install_was_successful(temp.env().path(), ".new_hooks");
}
