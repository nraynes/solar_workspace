use std::cell::Ref;

use mocked_up::{
    database::Database,
    rest_service::{Request, Response, Status},
};

use crate::resources::mock_download_api::{LATEST, db::COMMITALYZER_LINUX_BINARIES};

pub fn linux(_: Request, db: Ref<Database>) -> Response {
    if let Some(table) = db.table(COMMITALYZER_LINUX_BINARIES)
        && let Some(file) = table.data().get(LATEST)
    {
        return Response::new(file.clone(), Status::Ok);
    }
    Response::new(
        format!(
            "Database did not contain table '{}'!",
            COMMITALYZER_LINUX_BINARIES
        ),
        Status::InternalServerError,
    )
}
