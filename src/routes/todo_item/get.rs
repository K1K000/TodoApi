use crate::entities::prelude::TodoItem;
use crate::errorhand::{ErrorMessage, ErrorResponder};
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<sea_orm::JsonValue>>, ErrorResponder> {
    // type ORM = TodoItemGetORM;
    let db = db.inner();

    let todo_items = TodoItem::find()
        .into_json()
        .all(db)
        // .map(|val| ORM::from(val)) // GetORM should have Serialize implemented (derived)
        .await?;
    Ok(Json(todo_items))
}

#[get("/<id>")]
pub async fn by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<sea_orm::JsonValue>, ErrorResponder> {
    let db = db.inner();
    let todo_item = TodoItem::find_by_id(id)
        .into_json()
        .one(db)
        .await?
        .ok_or_else(|| {
            ErrorResponder::BadRequest(Json(ErrorMessage {
                message: "Record doesnt exist".into(),
            }))
        })?;
    Ok(Json(todo_item))
}
