use crate::entities::prelude::TodoItem;
use crate::entities::todo_item::{self, *};
use crate::errorhand::ErrorResponder;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

// routes/todo_item/mod.rs -> fn add_routes(r: Rocket<Build>) -> Rocket<Build>
// routes/todo_item/get.rs or something

//todo proper routing cause this is ass
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
) -> Result<Json<Option<sea_orm::JsonValue>>, ErrorResponder> {
    let db = db.inner();
    let todo_item = TodoItem::find_by_id(id).into_json().one(db).await?;
    Ok(Json(todo_item))
}

#[post("/", format = "json", data = "<new_item>")]
pub async fn post(
    db: &State<DatabaseConnection>,
    new_item: Json<Model>,
) -> Result<&'static str, ErrorResponder> {
    let db = db.inner();

    let new_item = todo_item::ActiveModel {
        name: Set(new_item.name.clone()),
        is_complete: Set(new_item.is_complete.clone()),
        ..Default::default()
    };
    TodoItem::insert(new_item)?;
    return "did do it this time";
}
