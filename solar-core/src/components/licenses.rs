pub mod installation;
mod installer;
pub mod license;
mod uninstaller;

pub use installer::LicensesInstaller;
pub use uninstaller::LicensesUninstaller;

pub static LICENSES_DIR: &str = "LICENSES";
