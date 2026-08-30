mod bin;
mod config;

use mocked_up::rest_service::RouteBuilderMut;

use crate::resources::mock_download_api::project_post_mapping;

pub fn semver_release_post_routes(b: RouteBuilderMut) -> RouteBuilderMut {
    project_post_mapping(
        b,
        "semver-release",
        "semver-release",
        "sample.config.semver.json",
        bin::arm_macos,
        bin::intel_macos,
        bin::linux,
        bin::windows,
        config::config,
    )
}
