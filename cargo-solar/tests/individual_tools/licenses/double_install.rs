use std::fs;

use rust_terminal::Terminal;
use solar_core::SOLARCONFIGNAME;

use crate::{
    individual_tools::licenses::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn double_install() {
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

    // Run second install
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
    println!("Checking second installation...");
    assert_installation(
        temp.env().path(),
        vec![
            "LICENSE-MIT",
            "LICENSE-Apache-2.0",
            "LICENSE-GPL-3.0",
            "LICENSE-NASA-1.3",
        ],
        vec![
            "LICENSE-MIT",
            "LICENSE-Apache-2.0",
            "LICENSE-MPL-1.0",
            "LICENSE-mailprio",
        ],
        false,
        false,
    );
    assert_configuration(
        temp.env().path(),
        vec!["MIT", "Apache-2.0", "GPL-3.0", "NASA-1.3"],
        vec!["MIT", "Apache-2.0", "MPL-1.0", "mailprio"],
        false,
    );
    println!("Second installation confirmed!");

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["uninstall", "licenses"])
        .unwrap();

    // Assert uninstalled correctly.
    println!("Checking uninstall...");
    assert!(!fs::exists(temp.env().path().join(SOLARCONFIGNAME)).unwrap());
    assert_installation(
        temp.env().path(),
        vec![
            "LICENSE-MIT",
            "LICENSE-Apache-2.0",
            "LICENSE-GPL-3.0",
            "LICENSE-NASA-1.3",
        ],
        vec![
            "LICENSE-MIT",
            "LICENSE-Apache-2.0",
            "LICENSE-MPL-1.0",
            "LICENSE-mailprio",
        ],
        true,
        true,
    );
    println!("Uninstall confirmed!");
}
