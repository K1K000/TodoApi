use crate::entities::prelude::TodoItem;
use crate::errorhand::{ErrorMessage, ErrorResponder};
use crate::routes::todo_item::dto::*;

use crate::entities::prelude::*;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;
// type ORM = TodoItemGetORM;
// .map(|val| ORM::from(val)) // GetORM should have Serialize implemented (derived)
//
#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTodoItem>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        TodoItem::find()
            .find_also_related(User)
            .all(db)
            .await?
            .iter()
            // TODO: handle the option properly
            .map(|(item, user)| {
                todo_item_to_dto(
                    item,
                    user.as_ref().expect("Item should have at least 1 user"),
                )
            })
            .collect(),
    ))
}

#[get("/<id>")]
pub async fn by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<ResponseTodoItem>, ErrorResponder> {
    let db = db.inner();
    let (item, user) = TodoItem::find_by_id(id)
        .find_also_related(User)
        .one(db)
        .await?
        .ok_or_else(|| {
            ErrorResponder::BadRequest(Json(ErrorMessage {
                message: "Record doesnt exist".into(),
            }))
        })?;

    let user = user.ok_or(ErrorResponder::NotFound(()))?;

    Ok(Json(todo_item_to_dto(&item, &user)))
    // Ok(Json(ResponseTodoItem {
    //     id: todo_item.id,
    //     name: todo_item.name,
    //     is_complete: todo_item.is_complete,
    //     user_id: todo_item.user_id,
    // }))
    // match serde_json::to_string(&ResponseTodoItem {
    //     id: todo_item.id,
    //     name: todo_item.name,
    //     is_complete: todo_item.is_complete,
    //     user_id: todo_item.user_id,
    // }) {
    //     Ok(val) => Ok(val),
    //     Err(_err) => Err(ErrorResponder::InternalError(Json(ErrorMessage {
    //         message: "server internal error".into(),
    //     }))),
}
