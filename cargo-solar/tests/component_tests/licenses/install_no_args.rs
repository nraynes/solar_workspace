use rust_terminal::Terminal;

use crate::setup_env;

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run install
    let install_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "install", "licenses"])
        .unwrap();

    // Assert install does nothing.
    assert!(
        String::from_utf8(install_output.stderr)
            .unwrap()
            .contains("No spdx identifiers supplied as arguments.")
    );
}
