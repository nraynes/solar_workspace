use rust_terminal::Terminal;

use crate::setup_env;

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run uninstall
    let uninstall_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "uninstall", "licenses"])
        .unwrap();

    // Assert uninstall does nothing (no install)
    assert!(
        String::from_utf8(uninstall_output.stderr)
            .unwrap()
            .contains("No such file or directory")
    );
}
