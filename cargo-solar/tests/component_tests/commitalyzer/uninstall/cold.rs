use rust_terminal::Terminal;

use crate::{
    component_tests::commitalyzer::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn commitalyzer() {
    let temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    let snapshot_before = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_before.is_git());
    assert!(snapshot_before.commit_msg_hook().is_none());
    assert!(snapshot_before.commit_rules().is_none());

    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "uninstall", "commitalyzer"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.is_git());
    assert!(snapshot_after.commit_msg_hook().is_none());
    assert!(snapshot_after.commit_rules().is_none());
}
