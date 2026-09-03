use affirm_fs::contains_subslice;
use rust_terminal::Terminal;

use crate::{
    component_tests::github_workflows::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn github_workflows() {
    let temp = setup_env();

    // Run command.
    let (stdout, stderr) = Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .stdout_and_stderr(CARGO_COMMAND, ["solar", "uninstall", "github-workflows"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert_eq!(stdout, "");
    assert!(contains_subslice(
        stderr.as_bytes(),
        "No such file or directory".as_bytes()
    ));

    assert!(snapshot_after.workflows_dir().is_none())
}
