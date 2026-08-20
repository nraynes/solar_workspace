use rust_terminal::Terminal;

use crate::{
    component_tests::vhooks::Snapshot,
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn vhooks() {
    let mut temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    // Get file system snapshot before command runs.
    let snapshot_before = Snapshot::from(temp.env().path().as_path());

    // Run command.
    let (stdout, stderr) = Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .stdout_and_stderr(CARGO_COMMAND, ["solar", "uninstall", "vhooks"])
        .unwrap();

    // Get file system snapshot after command runs.
    let snapshot_after = Snapshot::from(temp.env().path().as_path());

    assert_eq!(stdout, "".to_string());
    assert_eq!(stderr, "".to_string());
    assert_eq!(snapshot_before.is_git(), snapshot_after.is_git());

    let hooks_dir_before = snapshot_before.hooks_dir().as_ref().unwrap();
    let hooks_dir_after = snapshot_after.hooks_dir().as_ref().unwrap();

    assert!(hooks_dir_before.deep_eq().dir(&hooks_dir_after).unwrap());
}
