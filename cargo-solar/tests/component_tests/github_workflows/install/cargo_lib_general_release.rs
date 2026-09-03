use affirm_fs::contains_subslice;
use rust_terminal::Terminal;
use solar_core::{
    components::github_workflows::{
        WORKFLOW_ALREADY_EXISTS_ERROR_MESSAGE,
        installation::Installation,
        workflow::{CargoLibGeneralRelease, Workflow},
    },
    traits::GetPartialInstall,
};

use crate::{
    component_tests::github_workflows::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

static WORKFLOW_BEING_TESTED: &str = "cargo-lib-general-release";

#[test]
pub fn cold_hot() {
    let temp = setup_env();

    // Run command with default args.
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
                WORKFLOW_BEING_TESTED,
            ],
        )
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());
    let installation = Installation::get_current(temp.root().path()).unwrap();

    let expected_workflow = Workflow::CargoLibGeneralRelease(CargoLibGeneralRelease::new(
        "CI/CD Release".to_string(),
        "master".to_string(),
    ));

    let actual_workflow = snapshot_after
        .workflows_dir()
        .as_ref()
        .unwrap()
        .file("release.yml")
        .unwrap();

    assert_eq!(
        yaml_serde::to_string(&expected_workflow.build_yaml(&installation))
            .unwrap()
            .as_bytes(),
        actual_workflow.contents().unwrap()
    );

    // Run command with default args.
    let stdout = Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .stdout(
            CARGO_COMMAND,
            [
                "solar",
                "install",
                "github-workflows",
                "-f",
                "release.yml",
                WORKFLOW_BEING_TESTED,
            ],
        )
        .unwrap();

    assert!(contains_subslice(
        stdout.as_bytes(),
        WORKFLOW_ALREADY_EXISTS_ERROR_MESSAGE.as_bytes()
    ));
}

#[test]
pub fn cold_with_args() {
    let temp = setup_env();

    // Run command with default args.
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
                "release_phase.yml",
                WORKFLOW_BEING_TESTED,
                "--name",
                "Release Workflow",
                "--default-branch",
                "main",
            ],
        )
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());
    let installation = Installation::get_current(temp.root().path()).unwrap();

    let expected_workflow = Workflow::CargoLibGeneralRelease(CargoLibGeneralRelease::new(
        "Release Workflow".to_string(),
        "main".to_string(),
    ));

    let actual_workflow = snapshot_after
        .workflows_dir()
        .as_ref()
        .unwrap()
        .file("release_phase.yml")
        .unwrap();

    assert_eq!(
        yaml_serde::to_string(&expected_workflow.build_yaml(&installation))
            .unwrap()
            .as_bytes(),
        actual_workflow.contents().unwrap()
    );
}
