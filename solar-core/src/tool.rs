mod cargo_deny;
mod commitalyzer;
mod github_workflows;
mod licenses;
mod pre_commit;
mod semver_release;
mod vhooks;

use std::path::PathBuf;

pub use cargo_deny::CargoDeny;
pub use commitalyzer::Commitalyzer;
pub use github_workflows::Workflows;
pub use licenses::Licenses;
pub use pre_commit::PreCommit;
pub use semver_release::SemverRelease;
pub use vhooks::Vhooks;

use crate::SolarError;

use clap::{Parser, Subcommand as SC};

pub enum Action {
    INSTALL,
    UPGRADE,
    UNINSTALL,
}

pub trait ToolTrait {
    fn act(&mut self, action: &Action, dest: Option<PathBuf>) -> Result<(), SolarError> {
        if let Some(wd) = dest {
            self.set_dest(wd);
        }
        match action {
            Action::INSTALL => self.install(),
            Action::UPGRADE => self.upgrade(),
            Action::UNINSTALL => self.uninstall(),
        }
    }

    fn set_dest(&mut self, dest: PathBuf);

    fn install(&self) -> Result<(), SolarError>;

    fn uninstall(&self) -> Result<(), SolarError>;

    fn upgrade(&self) -> Result<(), SolarError> {
        self.uninstall()?;
        self.install()?;
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
    WORKFLOWS(Workflows),

    /// Configures project with a standard pre-commit hook for rust.
    PRECOMMIT(PreCommit),

    /// Configures project with a cargo deny for license checking.
    DENY(CargoDeny),
}

impl Tool {
    fn act(&mut self, action: &Action, dest: Option<PathBuf>) -> Result<(), SolarError> {
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

    pub fn perform(
        arg: Option<&mut Self>,
        action: Action,
        dest: Option<PathBuf>,
        pass_args: Vec<&str>,
    ) -> Result<(), SolarError> {
        match arg {
            Some(tool) => tool.act(&action, None),
            None => {
                Vhooks::parse_from(&pass_args).act(&action, dest.clone())?;
                Commitalyzer::parse_from(&pass_args).act(&action, dest.clone())?;
                SemverRelease::parse_from(&pass_args).act(&action, dest.clone())?;
                Licenses::parse_from(&pass_args).act(&action, dest.clone())?;
                Workflows::parse_from(&pass_args).act(&action, dest.clone())?;
                PreCommit::parse_from(&pass_args).act(&action, dest.clone())?;
                CargoDeny::parse_from(&pass_args).act(&action, dest.clone())?;
                Ok(())
            }
        }
    }
}
