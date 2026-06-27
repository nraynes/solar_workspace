use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at, individual_tools::cargo_deny::assert_installation,
    setup_env,
};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run install
    let install_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "install", "deny"])
        .unwrap();

    // Assert install does nothing
    assert!(
        String::from_utf8(install_output.stderr)
            .unwrap()
            .contains("No allowed licenses were specified.")
    );

    // Assert environment doesn't change.
    println!("Checking no installation...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None);
    println!("No installation confirmed!");
}
