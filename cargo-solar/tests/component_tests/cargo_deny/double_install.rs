use rust_terminal::Terminal;

use crate::{
    individual_tools::cargo_deny::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            [
                "solar",
                "install",
                "deny",
                "--allow-licenses",
                "MIT-1.0",
                "Unicode-3.0",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    assert_installation(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));
    assert_configuration(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));

    // Run second install
    let install_output = Terminal::command()
        .current_dir(temp.env().path())
        .run(
            "./cargo-solar",
            [
                "solar",
                "install",
                "deny",
                "--allow-licenses",
                "Apache-2.0",
                "GPL-3.0",
            ],
        )
        .unwrap();

    // Assert install does nothing
    assert!(
        String::from_utf8(install_output.stderr)
            .unwrap()
            .contains("Current installation found. Use cargo-deny to make changes to the current installation. Only use cargo-solar to uninstall completely.")
    );

    // Assert installed doesn't change.
    assert_installation(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));
    assert_configuration(temp.env().path(), Some(vec!["MIT-1.0", "Unicode-3.0"]));
}
