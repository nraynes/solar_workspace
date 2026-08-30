use affirm_fs::Directory;
use rust_terminal::Terminal;
use solar_core::tools::git::DEFAULT_GIT_HOOKS_DIR;

use crate::{
    component_tests::vhooks::{Snapshot, uninstall::ensure_install_succeeds_first},
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn vhooks() {
    let temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    ensure_install_succeeds_first(temp.root().path());

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(
            CARGO_COMMAND,
            ["solar", "uninstall", "vhooks", "--remove-hooks"],
        )
        .unwrap();

    // Get file system snapshot after command runs.
    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.is_git());

    let hooks_dir_after = snapshot_after.hooks_dir().as_ref().unwrap();

    // Assert that old directory has been removed.
    assert!(Directory::try_from(temp.root().path().join(".hooks")).is_err());

    assert_eq!(
        hooks_dir_after.path(),
        &temp.root().path().join(DEFAULT_GIT_HOOKS_DIR)
    );

    assert!(hooks_dir_after.directories().is_empty());
    assert!(hooks_dir_after.files().is_empty());
}
