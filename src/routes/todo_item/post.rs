use crate::entities::prelude::TodoItem;
use crate::entities::todo_item;
use crate::errorhand::ErrorResponder;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub name: String,
    pub is_complete: bool,
}

#[post("/", format = "json", data = "<new_item>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    new_item: Json<CreateTodoItem>,
) -> Result<&'static str, ErrorResponder> {
    let db = db.inner();

    let new_itemm = todo_item::ActiveModel {
        name: Set(new_item.name.clone()),
        is_complete: Set(new_item.is_complete),
        ..Default::default()
    };
    TodoItem::insert(new_itemm).exec(db).await?;
    Ok("did do it this time")
}
