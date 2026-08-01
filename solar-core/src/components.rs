pub mod cargo;
pub mod cargo_deny;
pub mod commitalyzer;
pub mod git;
pub mod github_workflows;
pub mod licenses;
pub mod pre_commit;
pub mod semver_release;
pub mod vhooks;

pub use cargo_deny::{CargoDenyInstaller, CargoDenyUninstaller};
pub use commitalyzer::{CommitalyzerInstaller, CommitalyzerUninstaller, CommitalyzerUpgrader};
pub use github_workflows::{GithubWorkflowsInstaller, GithubWorkflowsUninstaller};
pub use licenses::{LicensesInstaller, LicensesUninstaller};
pub use pre_commit::{PreCommitInstaller, PreCommitUninstaller};
pub use semver_release::{SemverReleaseInstaller, SemverReleaseUninstaller, SemverReleaseUpgrader};
pub use vhooks::{VhooksInstaller, VhooksUninstaller};

use clap::Subcommand;
use enum_dispatch::enum_dispatch;
use enum_printer::enum_printer;

#[enum_printer(
    InstallableComponent = [
        attributes = [
            enum_dispatch(Installable),
            derive(Subcommand, Clone, PartialEq, Debug)
        ],
        variants = [
            append_tuple(Installer)
        ],
    ],
    UninstallableComponent = [
        attributes = [
            enum_dispatch(Uninstallable),
            derive(Subcommand, Clone, PartialEq, Debug)
        ],
        variants = [
            append_tuple(Uninstaller)
        ],
    ],
    UpgradableComponent = [
        attributes = [
            enum_dispatch(Upgradable),
            derive(Subcommand, Clone, PartialEq, Debug)
        ],
        variants = [
            append_tuple(Upgrader)
        ],
    ],
)]
pub enum Component {
    /// Configures a versioned git hook folder for a project.
    #[print_to_enum(InstallableComponent, UninstallableComponent)]
    Vhooks(Vhooks),

    /// Installs commitalyzer (git commit linting tool) to the git hooks directory.
    #[print_to_enum(InstallableComponent, UninstallableComponent, UpgradableComponent)]
    Commitalyzer(Commitalyzer),

    /// Installs and configured SemVer-Release in the project.
    #[print_to_enum(InstallableComponent, UninstallableComponent, UpgradableComponent)]
    SemverRelease(SemverRelease),

    /// Installs the appropriate licenses into the project.
    #[print_to_enum(InstallableComponent, UninstallableComponent)]
    Licenses(Licenses),

    /// Configures project with standard Github workflows.
    #[print_to_enum(InstallableComponent, UninstallableComponent)]
    GithubWorkflows(GithubWorkflows),

    /// Configures project with a standard pre-commit hook for rust.
    #[print_to_enum(InstallableComponent, UninstallableComponent)]
    PreCommit(PreCommit),

    /// Configures project with a cargo deny for license checking.
    #[print_to_enum(InstallableComponent, UninstallableComponent)]
    CargoDeny(CargoDeny),
}
