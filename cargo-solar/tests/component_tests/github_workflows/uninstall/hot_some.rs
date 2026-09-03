use rust_terminal::Terminal;

use crate::{
    component_tests::github_workflows::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn exists() {
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
    let workflows_dir_before = snapshot_before.workflows_dir().as_ref().unwrap();

    assert!(workflows_dir_before.contains().file_named("test.yml"));
    assert!(workflows_dir_before.contains().file_named("release.yml"));

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(
            CARGO_COMMAND,
            [
                "solar",
                "uninstall",
                "github-workflows",
                "-f",
                "release.yml",
            ],
        )
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());
    let workflows_dir_after = snapshot_after.workflows_dir().as_ref().unwrap();

    assert!(workflows_dir_after.contains().file_named("test.yml"));
    assert!(!workflows_dir_after.contains().file_named("release.yml"));
}

#[test]
pub fn not_exists() {
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
    let workflows_dir_before = snapshot_before.workflows_dir().as_ref().unwrap();

    assert!(workflows_dir_before.contains().file_named("test.yml"));
    assert!(workflows_dir_before.contains().file_named("release.yml"));

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
                "not_exists.yml",
            ],
        )
        .unwrap();

    assert_eq!(stdout, "");
    assert_eq!(stderr, "");

    let snapshot_after = Snapshot::from(temp.root().path().as_path());
    let workflows_dir_after = snapshot_after.workflows_dir().as_ref().unwrap();

    assert!(workflows_dir_after.contains().file_named("test.yml"));
    assert!(workflows_dir_after.contains().file_named("release.yml"));
}
