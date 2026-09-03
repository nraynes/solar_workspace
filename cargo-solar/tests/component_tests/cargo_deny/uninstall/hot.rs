use rust_terminal::Terminal;

use crate::{
    component_tests::cargo_deny::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn cargo_deny() {
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

    let snapshot_before = Snapshot::from(temp.root().path().as_path());
    let deny_toml = snapshot_before.deny_toml().as_ref().unwrap();

    assert!(snapshot_before.crate_installed());
    assert_eq!(
        toml::to_string(deny_toml).unwrap(),
        "[licenses]\nallow = [\"MIT\", \"Apache-2.0\"]\n"
    );

    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "uninstall", "cargo-deny"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.deny_toml().is_none());
}
