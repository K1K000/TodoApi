use crate::entities::prelude::TodoItem;
use crate::entities::todo_item;
use crate::errorhand::ErrorResponder;
use crate::routes::todo_item::todo_item_dto::CreateTodoItem;
use rocket::http::Status;
// use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::{entity::*, query::*, *};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    new_item: Json<CreateTodoItem>,
    id: i32,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match TodoItem::find_by_id(id).one(db).await? {
        Some(_val) => {
            let new_itemm = todo_item::ActiveModel {
                name: Set(new_item.name.clone()),
                is_complete: Set(new_item.is_complete),
                ..Default::default()
            };
            todo_item::Entity::update(new_itemm)
                .filter(todo_item::Column::Id.contains(id.to_string()))
                .exec(db)
                .await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}
