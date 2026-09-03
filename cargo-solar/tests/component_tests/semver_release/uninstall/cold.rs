use crate::{
    component_tests::semver_release::uninstall::test_uninstall_was_successful, resources::setup_env,
};

#[test]
pub fn semver_release() {
    let temp = setup_env();

    test_uninstall_was_successful(temp.root().path(), None);
}
