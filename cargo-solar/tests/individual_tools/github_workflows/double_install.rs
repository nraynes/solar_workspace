use rust_terminal::Terminal;
use solar_core::tool::Workflow;

use crate::{
    individual_tools::github_workflows::{assert_configuration, assert_installation},
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
                "workflows",
                "--workflows-list",
                "release-cargo-bin-general",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    println!("Checking installation...");
    assert_installation(
        temp.env().path(),
        Some(vec![Workflow::ReleaseCargoBinGeneral]),
    );
    assert_configuration(
        temp.env().path(),
        Some(vec![Workflow::ReleaseCargoBinGeneral]),
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
                "workflows",
                "--workflows-list",
                "test-cargo-general",
            ],
        )
        .unwrap();

    // Assert installed performs as expected.
    println!("Checking second installation...");
    assert_installation(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
    );
    assert_configuration(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
    );
    println!("Second installation confirmed!");
}
