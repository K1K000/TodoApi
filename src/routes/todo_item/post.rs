use crate::entities::prelude::TodoItem;
use crate::entities::todo_item::{self, *};
use crate::errorhand::ErrorResponder;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[post("/", format = "json", data = "<new_item>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    new_item: Json<Model>,
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
