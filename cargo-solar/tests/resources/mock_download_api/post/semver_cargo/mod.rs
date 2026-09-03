mod bin;
mod config;

use mocked_up::rest_service::RouteBuilderMut;

use crate::resources::mock_download_api::project_post_mapping;

pub fn semver_cargo_post_routes(b: RouteBuilderMut) -> RouteBuilderMut {
    project_post_mapping(
        b,
        "semver-cargo",
        "semver-cargo",
        "sample.plugin.config.json",
        bin::arm_macos,
        bin::intel_macos,
        bin::linux,
        bin::windows,
        config::config,
    )
}
