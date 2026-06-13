use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{individual_tools::cargo_deny::assert_installation, setup_env};

#[test]
pub fn install_no_args() {
    let mut temp = setup_env();

    // Run install
    let install_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["install", "deny"])
        .unwrap();

    // Assert install does nothing
    assert!(
        String::from_utf8(install_output.stderr)
            .unwrap()
            .contains("No allowed licenses were specified.")
    );

    // Assert environment doesn't change.
    println!("Checking no installation...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    assert_installation(temp.env().path(), vec!["MIT-1.0", "Unicode-3.0"], true);
    println!("No installation confirmed!");
}
