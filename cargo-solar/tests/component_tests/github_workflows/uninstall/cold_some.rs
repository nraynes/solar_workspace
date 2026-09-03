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
        .stdout_and_stderr(
            CARGO_COMMAND,
            [
                "solar",
                "uninstall",
                "github-workflows",
                "-f",
                "test.yml",
                "release.yml",
            ],
        )
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert_eq!(stdout, "");
    assert_eq!(stderr, "");

    assert!(snapshot_after.workflows_dir().is_none())
}
