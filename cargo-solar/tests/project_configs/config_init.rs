use rust_terminal::Terminal;
use solar_core::tool::{Plugin, Ruleset, Workflow, pre_commit::Script};

use crate::{
    individual_tools::{
        cargo_deny, commitalyzer, github_workflows, licenses, pre_commit, semver_release, vhooks,
    },
    setup_env,
};

// #[test]
pub fn default() {
    let mut temp = setup_env();

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "init", "cargo-bin-basic"])
        .unwrap();

    // Assert configuration of all tools for this project configuration.
    cargo_deny::assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0", "Unicode-3.0"]),
    );
    commitalyzer::assert_configuration(temp.env().path(), &Some(Ruleset::ConventionalCommits));
    github_workflows::assert_configuration(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
    );
    licenses::assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0"]),
        Some(vec!["MIT", "Apache-2.0"]),
    );
    pre_commit::assert_configuration(temp.env().path(), Some(Script::CargoBasic));
    semver_release::assert_configuration(temp.env().path(), Some(vec![Plugin::Cargo]));
    vhooks::assert_configuration(temp.env().path(), ".hooks", true);

    // Assert installation of all tools for this project configuration.
    cargo_deny::assert_installation(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0", "Unicode-3.0"]),
    );
    commitalyzer::assert_installation(
        temp.env().path(),
        &Some(Ruleset::ConventionalCommits),
        true,
        true,
        true,
        true,
    );
    github_workflows::assert_installation(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
        true,
    );
    licenses::assert_installation(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0"]),
        Some(vec!["MIT", "Apache-2.0"]),
        true,
        true,
    );
    pre_commit::assert_installation(temp.env().path(), Some(Script::CargoBasic));
    semver_release::assert_installation(temp.env().path(), Some(vec![Plugin::Cargo]), true);
    vhooks::assert_installation(temp.env().path(), ".hooks", true);
}

// #[test]
pub fn overwrite() {
    let mut temp = setup_env();

    // Run individual tool installs
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run(
            "./cargo-solar",
            ["solar", "install", "vhooks", "--name", ".test_hooks"],
        )
        .unwrap();
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

    // Run install
    Terminal::command()
        .current_dir(temp.env().path())
        .piped()
        .run("./cargo-solar", ["solar", "init", "cargo-bin-basic"])
        .unwrap();

    // Assert that all previous configurations were uninstalled.
    vhooks::assert_installation(temp.env().path(), ".test_hooks", false);
    github_workflows::assert_installation(
        temp.env().path(),
        Some(vec![Workflow::ReleaseCargoLibGeneral]),
        false,
    );

    // Assert configuration of all tools for this project configuration.
    cargo_deny::assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0", "Unicode-3.0"]),
    );
    commitalyzer::assert_configuration(temp.env().path(), &Some(Ruleset::ConventionalCommits));
    github_workflows::assert_configuration(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
    );
    licenses::assert_configuration(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0"]),
        Some(vec!["MIT", "Apache-2.0"]),
    );
    pre_commit::assert_configuration(temp.env().path(), Some(Script::CargoBasic));
    semver_release::assert_configuration(temp.env().path(), Some(vec![Plugin::Cargo]));
    vhooks::assert_configuration(temp.env().path(), ".hooks", true);

    // Assert installation of all tools for this project configuration.
    cargo_deny::assert_installation(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0", "Unicode-3.0"]),
    );
    commitalyzer::assert_installation(
        temp.env().path(),
        &Some(Ruleset::ConventionalCommits),
        true,
        true,
        true,
        true,
    );
    github_workflows::assert_installation(
        temp.env().path(),
        Some(vec![
            Workflow::ReleaseCargoBinGeneral,
            Workflow::TestCargoGeneral,
        ]),
        true,
    );
    licenses::assert_installation(
        temp.env().path(),
        Some(vec!["MIT", "Apache-2.0"]),
        Some(vec!["MIT", "Apache-2.0"]),
        true,
        true,
    );
    pre_commit::assert_installation(temp.env().path(), Some(Script::CargoBasic));
    semver_release::assert_installation(temp.env().path(), Some(vec![Plugin::Cargo]), true);
    vhooks::assert_installation(temp.env().path(), ".hooks", true);
}
