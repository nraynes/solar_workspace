use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::commitalyzer::{RULESET_FOR_TESTING, assert_installation},
    setup_env,
};

#[test]
pub fn upgrade_no_args() {
    let mut temp = setup_env();

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["upgrade", "commitalyzer"])
        .unwrap();

    // Assert upgrade does nothing
    assert!(
        String::from_utf8(upgrade_output.stderr)
            .unwrap()
            .contains("A ruleset must be given for installation.")
    );

    // Assert environment doesn't change.
    println!("Checking no installation...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING.to_string()),
        false,
        false,
        false,
        false,
    );
    println!("No installation confirmed!");
}
