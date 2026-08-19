use rust_terminal::Terminal;

use crate::{component_tests::vhooks::install::test_vhooks_install, resources::setup_env};

// Just finished getting affirm-fs working and this first test is now passing. Time to make some more tests.

#[test]
pub fn vhooks() {
    let mut temp = setup_env();

    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("git", ["init"])
        .unwrap();

    test_vhooks_install(temp.env().path(), ".hooks");
}
