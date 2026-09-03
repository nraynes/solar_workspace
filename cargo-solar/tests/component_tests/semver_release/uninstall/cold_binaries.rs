use std::fs;

use solar_core::components::semver_release::RELEASE_DIR_NAME;

use crate::{
    component_tests::semver_release::{
        install::test_install_was_successful, uninstall::test_uninstall_was_successful,
    },
    resources::setup_env,
};

#[test]
pub fn semver_release() {
    let temp = setup_env();

    test_install_was_successful(temp.root().path(), None);

    fs::remove_dir_all(temp.root().path().join(RELEASE_DIR_NAME)).unwrap();

    test_uninstall_was_successful(temp.root().path(), None);
}
