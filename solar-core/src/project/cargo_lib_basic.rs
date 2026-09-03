use std::{fs::File, path::Path};

use clap::Parser;
use toml::Value;

use crate::{
    components::{
        CargoDenyInstaller, CargoDenyUninstaller, CommitalyzerInstaller, CommitalyzerUninstaller,
        CommitalyzerUpgrader, GithubWorkflowsInstaller, GithubWorkflowsUninstaller,
        LicensesInstaller, LicensesUninstaller, PreCommitInstaller, PreCommitUninstaller,
        SemverReleaseInstaller, SemverReleaseUninstaller, SemverReleaseUpgrader, VhooksInstaller,
        VhooksUninstaller,
        commitalyzer::ruleset::Ruleset,
        github_workflows::workflow::{CargoAnyGeneralTest, CargoLibGeneralRelease, Workflow},
        licenses::license::License,
        pre_commit::Script,
        semver_release::{Platform, Plugin},
    },
    solar_error::SolarError,
    tools::{cargo::CrateBuilder, git::set_remote_origin::set_remote_origin},
    traits::{ConfigureProject, Installable, Uninstallable, Upgradable},
};

#[derive(Parser, Clone)]
pub struct CargoLibBasic {
    /// The authors for this crate, if any.
    #[arg(short, long)]
    authors: Option<Vec<String>>,

    /// The description for this crate, if any.
    #[arg(short, long)]
    description: Option<String>,

    /// The repository for this crate, if any.
    #[arg(short, long)]
    repository: Option<String>,

    /// The keywords for this crate, if any.
    #[arg(short, long)]
    keywords: Option<Vec<String>>,

    /// The categories for this crate, if any.
    #[arg(short, long)]
    categories: Option<Vec<String>>,

    /// The git origin for this crate, if any.
    #[arg(short, long)]
    origin: Option<String>,

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
            CommitalyzerUninstaller::new().uninstall(path),
            SemverReleaseUninstaller::new(None).uninstall(path),
        ])
    }

    fn new(&self, path: &Path, name: &str) -> Result<(), SolarError> {
        // Initialize cargo bin package.
        let cratebuilder = CrateBuilder::new(
            path.join(name),
            name.into(),
            (0, 0, 0),
            self.authors.clone().unwrap_or(Vec::new()),
            self.description.clone().unwrap_or("".into()),
            "MIT OR Apache-2.0".into(),
            self.repository.clone().unwrap_or("".into()),
            self.keywords.clone().unwrap_or(Vec::new()),
            self.categories.clone().unwrap_or(Vec::new()),
            vec![],
        );
        cratebuilder.lib()?;

        // Create the readme.
        File::create(cratebuilder.path().join("README.md"))?;

        // Set the git origin point if provided.
        if let Some(origin) = &self.origin {
            set_remote_origin(cratebuilder.path(), origin)?;
        }

        self.init(cratebuilder.path())
    }

    fn init(&self, path: &Path) -> Result<(), SolarError> {
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
            SemverReleaseInstaller::new(Some(vec![Plugin::SemverCargo]), Platform::ArmMacos)
                .install(path),
        )?;

        // Update Cargo.toml to include new files in published crate.
        let mut cargo_toml = CrateBuilder::get_cargo_toml(path)?;
        let include_files_list = CrateBuilder::include_files_ref(&mut cargo_toml)?;
        include_files_list.extend(vec![
            Value::String("LICENSES/".into()),
            Value::String("LICENSE-MIT".into()),
            Value::String("LICENSE-Apache-2.0".into()),
            Value::String("CHANGELOG.md".into()),
        ]);
        CrateBuilder::save_cargo_toml(path, &cargo_toml)?;

        Ok(())
    }

    fn update(&self, path: &Path) -> Result<(), SolarError> {
        self.combine_errors(&[
            CommitalyzerUpgrader::new().upgrade(path),
            SemverReleaseUpgrader::new(Platform::ArmMacos).upgrade(path),
        ])
    }
}
