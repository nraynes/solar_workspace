use affirm_fs::Directory;
use rust_terminal::Terminal;
use solar_core::tools::git::DEFAULT_GIT_HOOKS_DIR;

use crate::{
    component_tests::vhooks::{Snapshot, uninstall::ensure_install_succeeds_first},
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn vhooks() {
    let mut temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    ensure_install_succeeds_first(temp.env().path());

    // Get file system snapshot before command runs.
    let snapshot_before = Snapshot::from(temp.env().path().as_path());

    // Run command.
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "uninstall", "vhooks"])
        .unwrap();

    // Get file system snapshot after command runs.
    let snapshot_after = Snapshot::from(temp.env().path().as_path());

    assert!(snapshot_after.is_git());

    let hooks_dir_before = snapshot_before.hooks_dir().as_ref().unwrap();
    let hooks_dir_after = snapshot_after.hooks_dir().as_ref().unwrap();

    // Assert that old directory has been removed.
    assert!(Directory::try_from(hooks_dir_before.path()).is_err());

    assert_eq!(
        hooks_dir_after.path(),
        &temp.env().path().join(DEFAULT_GIT_HOOKS_DIR)
    );

    assert!(
        hooks_dir_after
            .deep_eq()
            .dir_weak(hooks_dir_before)
            .unwrap()
    );
}
