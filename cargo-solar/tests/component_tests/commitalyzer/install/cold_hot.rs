use rust_terminal::Terminal;

use crate::{
    component_tests::commitalyzer::install::test_install_was_successful, resources::setup_env,
};

#[test]
pub fn commitalyzer() {
    let temp = setup_env();

    // Initialize Git first.
    Terminal::command()
        .current_dir(temp.root().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    test_install_was_successful(temp.root().path(), &[]);
    test_install_was_successful(temp.root().path(), &["conventional-commits"]);
    test_install_was_successful(temp.root().path(), &[]);
}
