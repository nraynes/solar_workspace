use rust_terminal::Terminal;
use solar_core::tool::Workflow;

use crate::{
    assert_configuration_file_does_not_exist_at,
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
                "test-cargo-general",
            ],
        )
        .unwrap();

    // Assert installed correctly.
    assert_installation(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
    );

    // Run upgrade
    let upgrade_output = Terminal::command()
        .current_dir(temp.env().path())
        .run("./cargo-solar", ["solar", "upgrade", "workflows"])
        .unwrap();

    // Assert upgrade does nothing (nothing to upgrade)
    assert!(
        String::from_utf8(upgrade_output.stdout)
            .unwrap()
            .contains("Nothing to upgrade for this tool.")
    );

    // Assert installed didn't change.
    assert_installation(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
        true,
    );
    assert_configuration(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
    );

    // Run uninstall
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "uninstall", "workflows"])
        .unwrap();

    // Assert uninstalled correctly.
    assert_configuration_file_does_not_exist_at(temp.env().path());
    assert_installation(temp.env().path(), None, true);
}
