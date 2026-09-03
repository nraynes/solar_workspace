use rust_terminal::Terminal;

use crate::{
    component_tests::licenses::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn licenses() {
    let temp = setup_env();

    let snapshot_before = Snapshot::from(temp.root().path().as_path());

    // Run command.
    let (stdout, stderr) = Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .stdout_and_stderr(CARGO_COMMAND, ["solar", "uninstall", "licenses"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert_eq!(
        snapshot_before.include_licenses().is_none(),
        snapshot_after.include_licenses().is_none()
    );
    assert!(
        snapshot_before
            .licensed_under()
            .eq()
            .dir(&snapshot_after.licensed_under())
    );

    assert_eq!(stdout, "");
    assert_eq!(stderr, "");
}
