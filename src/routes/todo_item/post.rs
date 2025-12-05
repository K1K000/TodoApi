use crate::entities::prelude::TodoItem;
use crate::entities::todo_item;
use crate::errorhand::ErrorResponder;
// use crate:routes::todo_item::post;
use crate::routes::todo_item::dto::{CreateTodoItem, ResponseTodoItem};
use rocket::response::status::Created;
// use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[post("/", format = "json", data = "<new_item>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    new_item: Json<CreateTodoItem>,
) -> Result<Created<Json<ResponseTodoItem>>, ErrorResponder> {
    let db = db.inner();

    let new_row = todo_item::ActiveModel {
        name: Set(new_item.name.clone()),
        is_complete: Set(new_item.is_complete),
        user_id: Set(new_item.user_id),
        ..Default::default()
    };
    let insert_res = TodoItem::insert(new_row).exec(db).await?;
    Ok(
        Created::new("http://127.0.0.1:8000/todoitem").body(Json(ResponseTodoItem {
            id: insert_res.last_insert_id,
            name: new_item.name.clone(),
            is_complete: new_item.is_complete,
            user_id: new_item.user_id,
            user_name: String::from("unknown at this point due to cost saving measures"),
        })),
    )
}
