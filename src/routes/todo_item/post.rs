use crate::entities::prelude::TodoItem;
use crate::entities::todo_item;
use crate::errorhand::ErrorResponder;
// use crate:routes::todo_item::post;
use crate::routes::todo_item::dto::CreateTodoItem;
// use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[post("/", format = "json", data = "<new_item>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    new_item: Json<CreateTodoItem>,
) -> Result<&'static str, ErrorResponder> {
    let db = db.inner();

    let new_row = todo_item::ActiveModel {
        name: Set(new_item.name.clone()),
        is_complete: Set(new_item.is_complete),
        user_id: Set(new_item.user_id),
        ..Default::default()
    };
    TodoItem::insert(new_row).exec(db).await?;
    Ok("did do it this time")
}
