#[macro_export]
macro_rules! get_download_url {
    ( $project:literal, $path:literal ) => {
        concat!(
            "https://github.com/nraynes/",
            $project,
            "/raw/refs/heads/master/",
            $path
        )
    };
}

pub static COMMITALYZER_BINARY_URL_ARM_MACOS: &str =
    get_download_url!("commitalyzer", "bin/arm-macos/commit-msg");
pub static COMMITALYZER_BINARY_URL_X86_MACOS: &str =
    get_download_url!("commitalyzer", "bin/intel-macos/commit-msg");
pub static COMMITALYZER_BINARY_URL_X86_LINUX: &str =
    get_download_url!("commitalyzer", "bin/linux/commit-msg");
pub static COMMITALYZER_BINARY_URL_X86_WINDOWS: &str =
    get_download_url!("commitalyzer", "bin/windows/commit-msg");
pub static COMMITALYZER_RULESET_BASE_URL: &str = get_download_url!("commitalyzer", "commit-rules");

pub static SEMVER_RELEASE_BINARY_URL_ARM_MACOS: &str =
    get_download_url!("semver-release", "bin/arm-macos/semver-release");
pub static SEMVER_RELEASE_BINARY_URL_X86_MACOS: &str =
    get_download_url!("semver-release", "bin/intel-macos/semver-release");
pub static SEMVER_RELEASE_BINARY_URL_X86_LINUX: &str =
    get_download_url!("semver-release", "bin/linux/semver-release");
pub static SEMVER_RELEASE_BINARY_URL_X86_WINDOWS: &str =
    get_download_url!("semver-release", "bin/windows/semver-release");
pub static SEMVER_RELEASE_CONFIG_URL: &str =
    get_download_url!("semver-release", "sample.config.semver.json");

pub static SEMVER_CARGO_BINARY_URL_ARM_MACOS: &str =
    get_download_url!("semver-cargo", "bin/arm-macos/semver-cargo");
pub static SEMVER_CARGO_BINARY_URL_X86_MACOS: &str =
    get_download_url!("semver-cargo", "bin/intel-macos/semver-cargo");
pub static SEMVER_CARGO_BINARY_URL_X86_LINUX: &str =
    get_download_url!("semver-cargo", "bin/linux/semver-cargo");
pub static SEMVER_CARGO_BINARY_URL_X86_WINDOWS: &str =
    get_download_url!("semver-cargo", "bin/windows/semver-cargo");
pub static SEMVER_CARGO_CONFIG_URL: &str =
    get_download_url!("semver-cargo", "sample.plugin.config.json");
