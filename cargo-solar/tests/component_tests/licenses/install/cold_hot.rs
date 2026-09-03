use crate::{
    component_tests::licenses::install::test_install_was_successful, resources::setup_env,
};

#[test]
pub fn licenses() {
    let temp = setup_env();

    test_install_was_successful(
        temp.root().path(),
        vec!["MIT", "Apache-2.0"],
        vec!["MIT", "Apache-2.0"],
    );
    test_install_was_successful(temp.root().path(), vec!["GPL-3.0"], vec!["GPL-3.0"]);
    test_install_was_successful(
        temp.root().path(),
        vec!["GPL-3.0", "X11"],
        vec!["GPL-3.0", "eCos-2.0", "xpp"],
    );
}
