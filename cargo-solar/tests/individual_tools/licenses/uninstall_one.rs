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
        vec!["LICENSE-MIT", "LICENSE-Apache-2.0"],
        vec!["LICENSE-MIT", "LICENSE-Apache-2.0"],
        false,
        false,
    );
    assert_configuration(
        temp.env().path(),
        vec!["MIT", "Apache-2.0"],
        vec!["MIT", "Apache-2.0"],
        false,
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
        vec!["LICENSE-Apache-2.0"],
        vec!["LICENSE-MIT"],
        false,
        false,
    );
    assert_installation(
        temp.env().path(),
        vec!["LICENSE-MIT"],
        vec!["LICENSE-Apache-2.0"],
        true,
        false,
    );
    assert_configuration(temp.env().path(), vec!["Apache-2.0"], vec!["MIT"], false);
    println!("Uninstall confirmed!");
}
