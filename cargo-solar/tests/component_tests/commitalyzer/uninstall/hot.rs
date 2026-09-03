use rust_terminal::Terminal;

use crate::{
    component_tests::commitalyzer::{Snapshot, install::test_install_was_successful},
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

    test_install_was_successful(temp.root().path(), &[]);

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
