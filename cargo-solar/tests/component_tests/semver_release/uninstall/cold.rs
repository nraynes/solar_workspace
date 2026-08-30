use rust_terminal::Terminal;

use crate::{
    component_tests::semver_release::{Snapshot, uninstall::test_uninstall_was_successful},
    resources::{CARGO_COMMAND, setup_env},
};

#[test]
pub fn semver_release() {
    let temp = setup_env();

    test_uninstall_was_successful(temp.root().path(), None);
}
