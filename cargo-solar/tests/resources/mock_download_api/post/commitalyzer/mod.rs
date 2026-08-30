mod bin;
mod config;

use mocked_up::rest_service::RouteBuilderMut;

use crate::resources::mock_download_api::project_post_mapping;

pub fn commitalyzer_post_routes(b: RouteBuilderMut) -> RouteBuilderMut {
    project_post_mapping(
        b,
        "commitalyzer",
        "commit-msg",
        "commit-rules",
        bin::arm_macos,
        bin::intel_macos,
        bin::linux,
        bin::windows,
        config::config,
    )
}
