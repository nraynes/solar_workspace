use rust_terminal::Terminal;

use crate::setup_env;

#[test]
pub fn uninstall_no_install() {
    let mut temp = setup_env();

    // Run uninstall
    let uninstall_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["uninstall", "licenses"])
        .unwrap();

    // Assert uninstall does nothing (no install)
    let errors = String::from_utf8(uninstall_output.stderr).unwrap();
    println!(
        "\n\nout: {}\n\n\nerr: {}\n\n",
        String::from_utf8(uninstall_output.stdout).unwrap(),
        &errors
    );
    assert!(errors.contains("No such file or directory"));
}
