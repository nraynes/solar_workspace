mod absolute_git_hooks_path;
mod copy_bin;
mod mock_download_api;
mod setup_env;

pub use absolute_git_hooks_path::absolute_git_hooks_path;
pub use copy_bin::copy_bin;
pub use mock_download_api::new_mock_download_api;
pub use setup_env::setup_env;

pub static CARGO_COMMAND: &str = "./cargo-solar";
