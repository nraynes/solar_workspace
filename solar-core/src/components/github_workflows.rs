pub mod installation;
mod installer;
mod uninstaller;
pub mod workflow;
pub mod yaml;

use std::path::{Path, PathBuf};

pub use installer::GithubWorkflowsInstaller;
pub use uninstaller::GithubWorkflowsUninstaller;

pub static WORKFLOW_PARSE_ERROR_MESSAGE: &str = "Must format workflow as '<file_name>:<workflow_name>:<workflow_options>;'.\n\nEXAMPLE: 'release:cargo-bin-general:name=CI/CD Release;main_branch=main;'";

fn workflows_path(path: &Path) -> PathBuf {
    path.join(".github/workflows")
}
