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
                "solar",
                "install",
                "licenses",
                "--include-licenses",
                "MIT",
                "Apache-2.0",
                "--licensed-under",
                "MIT",
                "Apache-2.0",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(
        temp.env().path(),
        Some(vec!["LICENSE-MIT", "LICENSE-Apache-2.0"]),
        Some(vec!["LICENSE-MIT", "LICENSE-Apache-2.0"]),
        true,
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0"]),
        Some(vec!["MIT", "Apache-2.0"]),
    );
    println!("Installation confirmed!");

    // Run second install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            [
                "solar",
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
    println!("Checking second installation...");
    assert_installation(
        temp.env().path(),
        Some(vec![
            "LICENSE-MIT",
            "LICENSE-Apache-2.0",
            "LICENSE-GPL-3.0",
            "LICENSE-NASA-1.3",
        ]),
        Some(vec![
            "LICENSE-MIT",
            "LICENSE-Apache-2.0",
            "LICENSE-MPL-1.0",
            "LICENSE-mailprio",
        ]),
        true,
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0", "GPL-3.0", "NASA-1.3"]),
        Some(vec!["MIT", "Apache-2.0", "MPL-1.0", "mailprio"]),
    );
    println!("Second installation confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "licenses"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstall...");
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, None, false, false);
    println!("Uninstall confirmed!");
}
