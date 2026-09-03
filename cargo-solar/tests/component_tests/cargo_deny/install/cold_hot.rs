use affirm_fs::contains_subslice;
use rust_terminal::Terminal;
use solar_core::components::cargo_deny::DENY_EXISTS_ERROR_MESSAGE;

use crate::{
    component_tests::cargo_deny::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn no_args() {
    let temp = setup_env();

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "install", "cargo-deny"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.crate_installed());
    assert!(snapshot_after.deny_toml().is_some());

    let stderr = Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .stderr(CARGO_COMMAND, ["solar", "install", "cargo-deny"])
        .unwrap();

    assert!(contains_subslice(
        stderr.as_bytes(),
        DENY_EXISTS_ERROR_MESSAGE.as_bytes()
    ))
}

#[test]
pub fn with_args() {
    let temp = setup_env();

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(
            CARGO_COMMAND,
            [
                "solar",
                "install",
                "cargo-deny",
                "--allow-licenses",
                "MIT",
                "Apache-2.0",
            ],
        )
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());
    let deny_toml = snapshot_after.deny_toml().as_ref().unwrap();

    assert!(snapshot_after.crate_installed());
    assert_eq!(
        toml::to_string(deny_toml).unwrap(),
        "[licenses]\nallow = [\"MIT\", \"Apache-2.0\"]\n"
    );
}
