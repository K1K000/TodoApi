use crate::entities::prelude::User;
use crate::errorhand::ErrorResponder;
use rocket::http::Status;
use rocket::*;
use sea_orm::{entity::*, *};

#[delete("/<id>")]
pub async fn by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Status, ErrorResponder> {
    let db = db.inner();

    match User::delete_by_id(id).exec(db).await? {
        DeleteResult { rows_affected: 1 } => Ok(Status::NoContent),
        _ => Err(ErrorResponder::NotFound(())),
    }
}
