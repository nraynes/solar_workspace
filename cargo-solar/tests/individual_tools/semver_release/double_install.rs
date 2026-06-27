use rust_terminal::Terminal;

use crate::{
    individual_tools::semver_release::{assert_configuration, assert_installation},
    setup_env,
};

#[test]
pub fn test() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "install", "semverrelease"])
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);
    println!("Installation confirmed!");

    // Run second install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "install", "semverrelease"])
        .unwrap();

    // Assert second install.
    println!("Checking second installation...");
    assert_installation(temp.env().path(), None, true);
    assert_configuration(temp.env().path(), None);
    println!("Second installation confirmed!");
}
