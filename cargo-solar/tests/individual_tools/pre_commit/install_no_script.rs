use std::fs;

use rust_terminal::Terminal;

use crate::{assert_configuration_file_does_not_exist_at, setup_env};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run upgrade.
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "install", "precommit"])
        .unwrap();

    // Assert install with no script arg errors with message.
    assert!(
        String::from_utf8(upgrade_output.stderr)
            .unwrap()
            .contains("No script value provided")
    );

    // Assert no install.
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert!(!fs::exists(temp.env().path().join(".git")).unwrap());
}
