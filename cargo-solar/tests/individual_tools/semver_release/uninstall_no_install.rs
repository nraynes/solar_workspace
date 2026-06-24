use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::semver_release::assert_installation, setup_env,
};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run uninstall
    let uninstall_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["uninstall", "semverrelease"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(uninstall_output.stderr)
            .unwrap()
            .contains("No such file or directory")
    );

    // Assert nothing changed.
    println!("Checking uninstallation...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, false);
    println!("Uninstallation confirmed!");
}
