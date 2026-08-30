mod db;
pub mod get;
pub mod post;
mod project_route_mapping;

pub use db::database;
pub use project_route_mapping::{project_get_mapping, project_post_mapping};

use mocked_up::rest_service::{RestBuilder, RestService};
use solar_core::solar_error::SolarError;

pub static LATEST: &str = "latest";

use crate::resources::mock_download_api::{
    get::{commitalyzer_get_routes, semver_cargo_get_routes, semver_release_get_routes},
    post::{commitalyzer_post_routes, semver_cargo_post_routes, semver_release_post_routes},
};

pub fn new_mock_download_api() -> Result<RestService, SolarError> {
    Ok(RestBuilder::new("https://github.com", database())?
        .get("nraynes", None, |b| {
            let b = commitalyzer_get_routes(b);
            let b = semver_release_get_routes(b);
            let b = semver_cargo_get_routes(b);
            b.build()
        })
        .post("nraynes", None, |b| {
            let b = commitalyzer_post_routes(b);
            let b = semver_release_post_routes(b);
            let b = semver_cargo_post_routes(b);
            b.build()
        })
        .build())
}
