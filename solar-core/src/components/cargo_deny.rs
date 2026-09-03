pub mod generate_toml;
pub mod installation;
mod installer;
mod uninstaller;

pub use installer::CargoDenyInstaller;
pub use uninstaller::CargoDenyUninstaller;

pub static CARGO_DENY_CRATE_NAME: &str = "cargo-deny";
pub static DENY_TOML_NAME: &str = "deny.toml";
pub static DENY_EXISTS_ERROR_MESSAGE: &str = "Current installation found. Use cargo-deny to make changes to the current installation. Only use cargo-solar to uninstall completely.";
