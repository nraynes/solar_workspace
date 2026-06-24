mod cargo_deny;
mod commitalyzer;
mod github_workflows;
mod licenses;
pub mod pre_commit;
mod semver_release;
mod vhooks;

use std::path::Path;

pub use cargo_deny::CargoDeny;
pub use commitalyzer::Commitalyzer;
pub use github_workflows::{GithubWorkflows, Parameters, Workflow};
pub use licenses::{LICENSES_DIR, Licenses};
pub use pre_commit::PreCommit;
pub use semver_release::{
    Plugin, RELEASE_BIN_NAME, RELEASE_CONFIG_NAME, RELEASE_DIR_NAME, SemverRelease,
};
pub use vhooks::Vhooks;

use crate::SolarError;

use clap::Subcommand as SC;

pub enum Action {
    INSTALL,
    UPGRADE,
    UNINSTALL,
}

pub trait ToolTrait {
    fn act(&mut self, action: &Action, dest: Option<&Path>) -> Result<(), SolarError> {
        if let Some(wd) = dest {
            self.set_dest(wd);
        }
        match action {
            Action::INSTALL => self.install(),
            Action::UPGRADE => self.upgrade(),
            Action::UNINSTALL => self.uninstall(),
        }
    }

    fn set_dest(&mut self, dest: &Path);

    fn install(&mut self) -> Result<(), SolarError>;

    fn uninstall(&mut self) -> Result<(), SolarError>;

    fn upgrade(&mut self) -> Result<(), SolarError> {
        println!("Nothing to upgrade for this tool.");
        Ok(())
    }
}

#[derive(SC, Clone, PartialEq, Debug)]
pub enum Tool {
    /// Configures a versioned git hook folder for a project.
    VHOOKS(Vhooks),

    /// Installs commitalyzer (git commit linting tool) to the git hooks directory.
    COMMITALYZER(Commitalyzer),

    /// Installs and configured SemVer-Release in the project.
    SEMVERRELEASE(SemverRelease),

    /// Installs the appropriate licenses into the project.
    LICENSES(Licenses),

    /// Configures project with standard Github workflows.
    WORKFLOWS(GithubWorkflows),

    /// Configures project with a standard pre-commit hook for rust.
    PRECOMMIT(PreCommit),

    /// Configures project with a cargo deny for license checking.
    DENY(CargoDeny),
}

impl Tool {
    pub fn act(&mut self, action: &Action, dest: Option<&Path>) -> Result<(), SolarError> {
        match self {
            Self::VHOOKS(tool) => tool.act(action, dest),
            Self::COMMITALYZER(tool) => tool.act(action, dest),
            Self::SEMVERRELEASE(tool) => tool.act(action, dest),
            Self::LICENSES(tool) => tool.act(action, dest),
            Self::WORKFLOWS(tool) => tool.act(action, dest),
            Self::PRECOMMIT(tool) => tool.act(action, dest),
            Self::DENY(tool) => tool.act(action, dest),
        }
    }
}
