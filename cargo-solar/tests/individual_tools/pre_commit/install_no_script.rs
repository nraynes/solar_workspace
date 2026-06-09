use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::setup_env;

#[test]
pub fn install_no_script() {
    let mut temp = setup_env();

    // Run upgrade.
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["install", "precommit"])
        .unwrap();

    // Assert install with no script arg errors with message.
    assert!(
        String::from_utf8(upgrade_output.stderr)
            .unwrap()
            .contains("No script value provided")
    );

    // Assert no install.
    println!("Checking no install...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    println!("No install confirmed!");
}
