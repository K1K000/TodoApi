use crate::entities::prelude::TodoItem;
use crate::errorhand::ErrorResponder;
use rocket::http::Status;
use rocket::*;
use sea_orm::{entity::*, *};

#[delete("/<id>")]
pub async fn by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    // this is the better way but i dont get the sea_orm::DeleteResult type :(

    match TodoItem::delete_by_id(id).exec(db).await? {
        DeleteResult { rows_affected: 1 } => Ok(Status::NoContent),
        _ => Err(ErrorResponder::NotFound(())),
    }
    // match TodoItem::find_by_id(id).one(db).await? {
    //     Some(val) => {
    //         val.delete(db).await?;
    //         Ok(Status::NoContent)
    //     }
    //     None => Err(ErrorResponder::NotFound(())),
    // }
}
