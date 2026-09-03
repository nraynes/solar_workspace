use std::cell::RefMut;

use mocked_up::{
    database::Database,
    rest_service::{Request, Response, Status},
};

use crate::resources::mock_download_api::{LATEST, db::COMMITALYZER_INTEL_MACOS_BINARIES};

pub fn intel_macos(req: Request, mut db: RefMut<Database>) -> Response {
    let new_file = req.body();
    if let Some(table) = db.table_mut(COMMITALYZER_INTEL_MACOS_BINARIES) {
        table.data_mut().remove(LATEST);
        table.data_mut().insert(LATEST.into(), new_file.clone());
        return Response::new("".into(), Status::Ok);
    }
    Response::new(
        format!(
            "Database did not contain table '{}'!",
            COMMITALYZER_INTEL_MACOS_BINARIES
        ),
        Status::InternalServerError,
    )
}
