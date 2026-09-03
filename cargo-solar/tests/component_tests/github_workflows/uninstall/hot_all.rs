use rust_terminal::Terminal;

use crate::{
    component_tests::github_workflows::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn github_workflows() {
    let temp = setup_env();

    // Install test workflow.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(
            CARGO_COMMAND,
            [
                "solar",
                "install",
                "github-workflows",
                "-f",
                "test.yml",
                "cargo-any-general-test",
            ],
        )
        .unwrap();

    // Install release workflow.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(
            CARGO_COMMAND,
            [
                "solar",
                "install",
                "github-workflows",
                "-f",
                "release.yml",
                "cargo-bin-general-release",
            ],
        )
        .unwrap();

    let snapshot_before = Snapshot::from(temp.root().path().as_path());
    let workflows_dir = snapshot_before.workflows_dir().as_ref().unwrap();

    assert!(workflows_dir.contains().file_named("test.yml"));
    assert!(workflows_dir.contains().file_named("release.yml"));

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "uninstall", "github-workflows"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.workflows_dir().is_none())
}
