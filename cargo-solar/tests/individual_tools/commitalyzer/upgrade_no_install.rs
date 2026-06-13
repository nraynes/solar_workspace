use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{
    individual_tools::commitalyzer::{RULESET_FOR_TESTING, assert_installation},
    setup_env,
};

#[test]
pub fn uninstall_no_install() {
    let mut temp = setup_env();

    // Run uninstall
    let uninstall_output = Terminal::command()
        .current_dir(temp.env().path())
        .run(
            "./cargo-solar",
            [
                "upgrade",
                "commitalyzer",
                "--ruleset",
                "conventional-commits",
            ],
        )
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(uninstall_output.stderr)
            .unwrap()
            .contains("No such file or directory")
    );

    // Assert environment doesn't change.
    println!("Checking no installation...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING.to_string()),
        true,
        true,
    );
    println!("No installation confirmed!");
}
