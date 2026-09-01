pub mod installation;
mod installer;
mod script;
mod uninstaller;

pub use installer::{PRECOMMIT_ALREADY_EXISTS_ERR_MSG, PreCommitInstaller};
pub use script::Script;
pub use uninstaller::PreCommitUninstaller;

pub static PRE_COMMIT: &str = "pre-commit";
