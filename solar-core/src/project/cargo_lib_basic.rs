use std::path::Path;

use clap::Parser;

use crate::{
    components::{
        CargoDenyInstaller, CargoDenyUninstaller, CommitalyzerInstaller, CommitalyzerUninstaller,
        CommitalyzerUpgrader, GithubWorkflowsInstaller, GithubWorkflowsUninstaller,
        LicensesInstaller, LicensesUninstaller, PreCommitInstaller, PreCommitUninstaller,
        SemverReleaseInstaller, SemverReleaseUninstaller, SemverReleaseUpgrader, VhooksInstaller,
        VhooksUninstaller,
        commitalyzer::ruleset::Ruleset,
        git::init::try_git_init,
        github_workflows::workflow::{CargoAnyGeneralTest, CargoLibGeneralRelease, Workflow},
        licenses::license::License,
        pre_commit::Script,
        semver_release::Plugin,
    },
    solar_error::SolarError,
    traits::{ConfigureProject, Installable, Uninstallable, Upgradable},
};

#[derive(Parser, Clone)]
pub struct CargoLibBasic {
    /// If there is already a pre-commit hook present, this option will allow it to be overwritten.
    #[arg(short)]
    force_overwrite_pre_commit: bool,
}

impl ConfigureProject for CargoLibBasic {
    fn deinit(&self, path: &Path) -> Result<(), SolarError> {
        self.combine_errors(&[
            CargoDenyUninstaller::new().uninstall(path),
            VhooksUninstaller::new(false).uninstall(path),
            PreCommitUninstaller::new().uninstall(path),
            LicensesUninstaller::new(None, None).uninstall(path),
            GithubWorkflowsUninstaller::new(Some(vec!["test".into(), "release".into()]))
                .uninstall(path),
            CommitalyzerUninstaller::new(None).uninstall(path),
            SemverReleaseUninstaller::new(None).uninstall(path),
        ])
    }

    fn init(&self, path: &Path) -> Result<(), SolarError> {
        // Initialize git if it hasn't been already.
        try_git_init(path)?;

        // Install cargo-deny.
        self.clean_up_on_error(
            path,
            CargoDenyInstaller::new(vec![
                "MIT".into(),
                "Apache-2.0".into(),
                "Unicode-3.0".into(),
            ])
            .install(path),
        )?;

        // Install Vhooks.
        self.clean_up_on_error(
            path,
            VhooksInstaller::new(path.join(".hooks")).install(path),
        )?;

        // Install pre-commit hook.
        self.clean_up_on_error(
            path,
            PreCommitInstaller::new(Script::CargoBasic, self.force_overwrite_pre_commit)
                .install(path),
        )?;

        // Install licenses.
        self.clean_up_on_error(
            path,
            LicensesInstaller::new(
                Some(vec![License::MIT, License::Apache2x0, License::Unicode3x0]),
                vec![License::MIT, License::Apache2x0],
            )
            .install(path),
        )?;

        // Install Github workflows.
        self.clean_up_on_error(
            path,
            GithubWorkflowsInstaller::new(
                Workflow::CargoLibGeneralRelease(CargoLibGeneralRelease::new(
                    "CI/CD Release".into(),
                    "master".into(),
                )),
                "release".into(),
            )
            .install(path),
        )?;
        self.clean_up_on_error(
            path,
            GithubWorkflowsInstaller::new(
                Workflow::CargoAnyGeneralTest(CargoAnyGeneralTest::new(
                    "CI/CD Test".into(),
                    "master".into(),
                )),
                "test".into(),
            )
            .install(path),
        )?;

        // Install Commitalyzer.
        self.clean_up_on_error(
            path,
            CommitalyzerInstaller::new(Some(vec![Ruleset::ConventionalCommits])).install(path),
        )?;

        // Install SemverRelease.
        self.clean_up_on_error(
            path,
            SemverReleaseInstaller::new(Some(vec![Plugin::Cargo])).install(path),
        )?;

        Ok(())
    }

    fn update(&self, path: &Path) -> Result<(), SolarError> {
        self.combine_errors(&[
            CommitalyzerUpgrader::new().upgrade(path),
            SemverReleaseUpgrader::new().upgrade(path),
        ])
    }
}
