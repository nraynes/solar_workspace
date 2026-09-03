use rust_terminal::Terminal;

use crate::{
    component_tests::licenses::{Snapshot, install::test_install_was_successful},
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn licenses() {
    let temp = setup_env();

    test_install_was_successful(
        temp.root().path(),
        vec!["MIT", "Apache-2.0", "GPL-3.0", "X11", "eCos-2.0", "xpp"],
        vec!["MIT", "Apache-2.0", "GPL-3.0", "X11", "eCos-2.0", "xpp"],
    );

    // Run command.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run(CARGO_COMMAND, ["solar", "uninstall", "licenses"])
        .unwrap();

    let snapshot_after = Snapshot::from(temp.root().path().as_path());

    assert!(snapshot_after.include_licenses().is_none());
    assert!(snapshot_after.licensed_under().files().is_empty());
}
