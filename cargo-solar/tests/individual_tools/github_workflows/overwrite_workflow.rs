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
                "solar",
                "install",
                "workflows",
                "--workflows-list",
                "release-cargo-bin-general",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    assert_installation(
        temp.env().path(),
        Some(vec![Workflow::ReleaseCargoBinGeneral]),
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec![Workflow::ReleaseCargoBinGeneral]),
    );

    // Run second install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            [
                "solar",
                "install",
                "workflows",
                "--workflows-list",
                "release-cargo-lib-general",
            ],
        )
        .unwrap();

    // Assert installed performs as expected.
    assert_installation(
        temp.env().path(),
        Some(vec![Workflow::ReleaseCargoLibGeneral]),
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec![Workflow::ReleaseCargoLibGeneral]),
    );
}
