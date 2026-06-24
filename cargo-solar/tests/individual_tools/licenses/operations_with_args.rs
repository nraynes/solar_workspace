use rust_terminal::Terminal;

use crate::{
    assert_configuration_file_does_not_exist_at,
    individual_tools::licenses::{assert_configuration, assert_installation},
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
                "install",
                "licenses",
                "--include-licenses",
                "GPL-3.0",
                "NASA-1.3",
                "--licensed-under",
                "MPL-1.0",
                "mailprio",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(
        temp.env().path(),
        Some(vec!["LICENSE-GPL-3.0", "LICENSE-NASA-1.3"]),
        Some(vec!["LICENSE-MPL-1.0", "LICENSE-mailprio"]),
        true,
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec!["GPL-3.0", "NASA-1.3"]),
        Some(vec!["MPL-1.0", "mailprio"]),
    );
    println!("Installation confirmed!");

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run(
            "./cargo-solar",
            [
                "upgrade",
                "licenses",
                "--include-licenses",
                "GPL-3.0",
                "NASA-1.3",
                "--licensed-under",
                "MPL-1.0",
                "mailprio",
            ],
        )
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installed doesn't change.
    println!("Checking upgrade...");
    assert_installation(
        temp.env().path(),
        Some(vec!["LICENSE-GPL-3.0", "LICENSE-NASA-1.3"]),
        Some(vec!["LICENSE-MPL-1.0", "LICENSE-mailprio"]),
        true,
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec!["GPL-3.0", "NASA-1.3"]),
        Some(vec!["MPL-1.0", "mailprio"]),
    );
    println!("Upgrade confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["uninstall", "licenses"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstall...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, None, false, false);
    println!("Uninstall confirmed!");
}
