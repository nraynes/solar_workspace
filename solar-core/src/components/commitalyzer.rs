pub mod download;
pub mod installation;
mod installer;
pub mod ruleset;
mod uninstaller;
mod upgrader;

pub use installer::CommitalyzerInstaller;
pub use uninstaller::CommitalyzerUninstaller;
pub use upgrader::CommitalyzerUpgrader;

pub static COMMIT_MSG_NAME: &str = "commit-msg";
pub static COMMIT_RULES_NAME: &str = "commit-rules";
