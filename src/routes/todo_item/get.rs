use crate::entities::prelude::TodoItem;
use crate::errorhand::{ErrorMessage, ErrorResponder};
use crate::routes::todo_item::dto::*;
use futures::future;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;
// type ORM = TodoItemGetORM;
// .map(|val| ORM::from(val)) // GetORM should have Serialize implemented (derived)

//dont look at this bs
#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<sea_orm::JsonValue>>, ErrorResponder> {
    let db = db.inner();

    let todo_items = TodoItem::find().into_json().all(db).await?;
    // let json_todo_items =
    //     future::try_join_all(todo_items.iter().map(|val| todo_item_to_dto(val.clone())))
    //         .await?
    //         .unwrap();

    // .(|val| Json(todo_item_to_dto(val.clone()).await()));
    Ok(Json(todo_items))
    // Ok(Json(
    //     todo_items
    //         .iter()
    //         .map(|f| todo_item_to_dto(f.clone()).await())
    //         .collect(),
    // ))
}

#[get("/<id>")]
pub async fn by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<ResponseTodoItem>, ErrorResponder> {
    let db = db.inner();
    let todo_item = TodoItem::find_by_id(id).one(db).await?.ok_or_else(|| {
        ErrorResponder::BadRequest(Json(ErrorMessage {
            message: "Record doesnt exist".into(),
        }))
    })?;
    Ok(Json(todo_item_to_dto(todo_item).await))
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
