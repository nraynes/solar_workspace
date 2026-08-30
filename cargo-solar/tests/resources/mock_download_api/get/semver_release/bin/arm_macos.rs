use std::cell::Ref;

use mocked_up::{
    database::Database,
    rest_service::{Request, Response, Status},
};

use crate::resources::mock_download_api::{LATEST, db::SEMVER_RELEASE_ARM_MACOS_BINARIES};

pub fn arm_macos(_: Request, db: Ref<Database>) -> Response {
    if let Some(table) = db.table(SEMVER_RELEASE_ARM_MACOS_BINARIES)
        && let Some(file) = table.data().get(LATEST)
    {
        return Response::new(file.clone(), Status::Ok);
    }
    Response::new(
        format!(
            "Database did not contain table '{}'!",
            SEMVER_RELEASE_ARM_MACOS_BINARIES
        ),
        Status::InternalServerError,
    )
}
