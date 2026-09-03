use rust_terminal::Terminal;

use crate::{
    component_tests::cargo_deny::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn cargo_deny() {
    let temp = setup_env();

    let (stdout, stderr) = Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .stdout_and_stderr(CARGO_COMMAND, ["solar", "uninstall", "cargo-deny"])
        .unwrap();

    assert_eq!(stdout, "");
    assert_eq!(stderr, "");

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.deny_toml().is_none());
}
