use rust_terminal::Terminal;

use crate::{
    individual_tools::commitalyzer::{
        RULESET_FOR_TESTING, assert_configuration, assert_installation,
    },
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
                "commitalyzer",
                "--ruleset",
                "conventional-commits",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING.to_string()),
        true,
        true,
        true,
        true,
    );
    assert_configuration(temp.env().path(), &Some(RULESET_FOR_TESTING));
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
                "commitalyzer",
                "--ruleset",
                "conventional-commits",
            ],
        )
        .unwrap();

    // Assert installed performs as expected.
    println!("Checking second installation...");
    assert_installation(
        temp.env().path(),
        &Some(RULESET_FOR_TESTING.to_string()),
        true,
        true,
        true,
        true,
    );
    assert_configuration(temp.env().path(), &Some(RULESET_FOR_TESTING));
    println!("Second installation confirmed!");
}
