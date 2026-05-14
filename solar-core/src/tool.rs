mod cargo_deny;
mod commitalyzer;
mod github_workspaces;
mod licenses;
mod pre_commit;
mod semver_release;
mod vhooks;

pub use cargo_deny::CargoDeny;
pub use commitalyzer::Commitalyzer;
pub use github_workspaces::Workspaces;
pub use licenses::Licenses;
pub use pre_commit::PreCommit;
pub use semver_release::SemverRelease;
pub use vhooks::Vhooks;

use crate::SolarError;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use clap::Subcommand as SC;

pub enum Action {
    INSTALL,
    UPGRADE,
    REMOVE,
}

pub trait ToolTrait {
    fn act(&self, action: &Action) -> Result<(), SolarError> {
        match action {
            Action::INSTALL => self.install(),
            Action::UPGRADE => self.upgrade(),
            Action::REMOVE => self.remove(),
        }
    }

    fn install(&self) -> Result<(), SolarError>;

    fn remove(&self) -> Result<(), SolarError>;

    fn upgrade(&self) -> Result<(), SolarError> {
        self.remove()?;
        self.install()?;
        Ok(())
    }
}

#[derive(SC, Clone, EnumIter, PartialEq, Debug)]
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
    WORKSPACES(Workspaces),

    /// Configures project with a standard pre-commit hook for rust.
    PRECOMMIT(PreCommit),

    /// Configures project with a cargo deny for license checking.
    DENY(CargoDeny),
}

impl Tool {
    fn act(&self, action: &Action) -> Result<(), SolarError> {
        match self {
            Self::VHOOKS(tool) => tool.act(action),
            Self::COMMITALYZER(tool) => tool.act(action),
            Self::SEMVERRELEASE(tool) => tool.act(action),
            Self::LICENSES(tool) => tool.act(action),
            Self::WORKSPACES(tool) => tool.act(action),
            Self::PRECOMMIT(tool) => tool.act(action),
            Self::DENY(tool) => tool.act(action),
        }
    }

    pub fn perform(arg: &Option<Self>, action: Action) -> Result<(), SolarError> {
        match arg {
            Some(tool) => tool.act(&action),
            None => {
                for tool in Self::iter() {
                    tool.act(&action)?;
                }
                Ok(())
            }
        }
    }
}
