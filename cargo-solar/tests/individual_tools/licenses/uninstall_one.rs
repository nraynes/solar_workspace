use rust_terminal::Terminal;

use crate::{
    individual_tools::licenses::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn uninstall_one() {
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

    // Uninstall one license from each.
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            [
                "uninstall",
                "licenses",
                "--include-licenses",
                "MIT",
                "--licensed-under",
                "Apache-2.0",
            ],
        )
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstall...");
    assert_installation(
        temp.env().path(),
        Some(vec!["LICENSE-Apache-2.0"]),
        Some(vec!["LICENSE-MIT"]),
        true,
        true,
    );
    assert_installation(
        temp.env().path(),
        Some(vec!["LICENSE-MIT"]),
        Some(vec!["LICENSE-Apache-2.0"]),
        false,
        false,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec!["Apache-2.0"]),
        Some(vec!["MIT"]),
    );
    println!("Uninstall confirmed!");
}
