use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::github_workflows::assert_installation, setup_env,
};

#[test]
pub fn install_no_args() {
    let mut temp = setup_env();

    // Run install
    let install_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["install", "workflows"])
        .unwrap();

    // Assert install does nothing
    assert!(
        String::from_utf8(install_output.stderr)
            .unwrap()
            .contains("At least one workflow must be given for installation.")
    );

    // Assert environment doesn't change.
    println!("Checking no installation...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None);
    println!("No installation confirmed!");
}
