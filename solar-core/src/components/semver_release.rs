pub mod download;
pub mod installation;
mod installer;
mod platform;
mod plugin;
mod uninstaller;
mod upgrader;

pub use installer::SemverReleaseInstaller;
pub use platform::Platform;
pub use plugin::Plugin;
pub use uninstaller::SemverReleaseUninstaller;
pub use upgrader::SemverReleaseUpgrader;

pub static RELEASE_DIR_NAME: &str = ".release";
pub static RELEASE_BIN_NAME: &str = "semver-release";
pub static RELEASE_CONFIG_NAME: &str = "config.semver.json";
pub static CONFIG_PLUGINS_SECTION: &str = "plugins";
