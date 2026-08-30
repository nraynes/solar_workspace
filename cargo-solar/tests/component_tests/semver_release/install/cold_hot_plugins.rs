use crate::{
    component_tests::semver_release::install::test_install_was_successful, resources::setup_env,
};

#[test]
pub fn semver_release() {
    let temp = setup_env();

    test_install_was_successful(temp.root().path(), Some(vec!["semver-cargo"]));
    test_install_was_successful(temp.root().path(), Some(vec!["semver-cargo"]));
}
