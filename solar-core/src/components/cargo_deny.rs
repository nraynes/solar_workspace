pub mod generate_toml;
pub mod installation;
mod installer;
mod uninstaller;

pub use installer::CargoDenyInstaller;
pub use uninstaller::CargoDenyUninstaller;

pub static CARGO_DENY_CRATE_NAME: &str = "cargo-deny";
pub static DENY_TOML_NAME: &str = "deny.toml";
