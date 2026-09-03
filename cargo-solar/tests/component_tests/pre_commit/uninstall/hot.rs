use rust_terminal::Terminal;

use crate::{
    component_tests::pre_commit::{Snapshot, install::test_install_was_successful},
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn pre_commit() {
    let temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    test_install_was_successful(temp.root().path(), "cargo-basic", false);

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "uninstall", "pre-commit"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.is_git());
    assert!(snapshot_after.pre_commit_file().is_none());
}
