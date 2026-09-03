use rust_terminal::Terminal;

use crate::{
    component_tests::pre_commit::install::test_install_was_successful, resources::setup_env,
};

#[test]
pub fn pre_commit() {
    let temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    test_install_was_successful(temp.root().path(), "cargo-basic", false);
    test_install_was_successful(temp.root().path(), "cargo-basic", false);
    test_install_was_successful(temp.root().path(), "cargo-basic", true);
}
