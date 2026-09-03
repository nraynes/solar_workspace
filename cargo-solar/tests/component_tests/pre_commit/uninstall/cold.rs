use rust_terminal::Terminal;

use crate::{
    component_tests::pre_commit::Snapshot,
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

    let snapshot_before = Snapshot::from(temp.root().path().as_path());

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "uninstall", "pre-commit"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert_eq!(snapshot_before.is_git(), snapshot_after.is_git());
    assert_eq!(
        snapshot_before.pre_commit_file().is_none(),
        snapshot_before.pre_commit_file().is_none()
    );
}
