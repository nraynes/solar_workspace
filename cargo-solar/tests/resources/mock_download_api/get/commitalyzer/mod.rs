mod bin;
mod config;

use mocked_up::rest_service::RouteBuilder;

use crate::resources::mock_download_api::project_get_mapping;

pub fn commitalyzer_get_routes(b: RouteBuilder) -> RouteBuilder {
    project_get_mapping(
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
