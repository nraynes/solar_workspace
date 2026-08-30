pub mod generate_toml;
pub mod installation;
mod installer;
mod uninstaller;

pub use installer::CargoDenyInstaller;
pub use uninstaller::CargoDenyUninstaller;

pub static PKG_NAME: &str = "cargo-deny";
pub static TOML_NAME: &str = "deny.toml";
