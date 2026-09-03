use mocked_up::database::Database;

pub static COMMITALYZER_ARM_MACOS_BINARIES: &str = "commitalyzer_arm_macos_binaries";
pub static COMMITALYZER_INTEL_MACOS_BINARIES: &str = "commitalyzer_intel_macos_binaries";
pub static COMMITALYZER_LINUX_BINARIES: &str = "commitalyzer_linux_binaries";
pub static COMMITALYZER_WINDOWS_BINARIES: &str = "commitalyzer_windows_binaries";
pub static COMMITALYZER_COMMIT_RULES: &str = "commitalyzer_commit_rules";

pub static SEMVER_RELEASE_ARM_MACOS_BINARIES: &str = "semver_release_arm_macos_binaries";
pub static SEMVER_RELEASE_INTEL_MACOS_BINARIES: &str = "semver_release_intel_macos_binaries";
pub static SEMVER_RELEASE_LINUX_BINARIES: &str = "semver_release_linux_binaries";
pub static SEMVER_RELEASE_WINDOWS_BINARIES: &str = "semver_release_windows_binaries";
pub static SEMVER_RELEASE_CONFIG_FILE: &str = "semver_release_config_file";

pub static SEMVER_CARGO_ARM_MACOS_BINARIES: &str = "semver_cargo_arm_macos_binaries";
pub static SEMVER_CARGO_INTEL_MACOS_BINARIES: &str = "semver_cargo_intel_macos_binaries";
pub static SEMVER_CARGO_LINUX_BINARIES: &str = "semver_cargo_linux_binaries";
pub static SEMVER_CARGO_WINDOWS_BINARIES: &str = "semver_cargo_windows_binaries";
pub static SEMVER_CARGO_CONFIG_FILE: &str = "semver_cargo_config_file";

pub fn database() -> Database {
    Database::new()
        .add_table(COMMITALYZER_ARM_MACOS_BINARIES)
        .add_table(COMMITALYZER_INTEL_MACOS_BINARIES)
        .add_table(COMMITALYZER_LINUX_BINARIES)
        .add_table(COMMITALYZER_WINDOWS_BINARIES)
        .add_table(COMMITALYZER_COMMIT_RULES)
        .add_table(SEMVER_RELEASE_ARM_MACOS_BINARIES)
        .add_table(SEMVER_RELEASE_INTEL_MACOS_BINARIES)
        .add_table(SEMVER_RELEASE_LINUX_BINARIES)
        .add_table(SEMVER_RELEASE_WINDOWS_BINARIES)
        .add_table(SEMVER_RELEASE_CONFIG_FILE)
        .add_table(SEMVER_CARGO_ARM_MACOS_BINARIES)
        .add_table(SEMVER_CARGO_INTEL_MACOS_BINARIES)
        .add_table(SEMVER_CARGO_LINUX_BINARIES)
        .add_table(SEMVER_CARGO_WINDOWS_BINARIES)
        .add_table(SEMVER_CARGO_CONFIG_FILE)
}
