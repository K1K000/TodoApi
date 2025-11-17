use crate::entities::prelude::TodoItem;
use crate::errorhand::ErrorResponder;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

//todo proper routing cause this is ass
#[get("/todoitem")]
pub async fn get_todo_items(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<String>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let todo_items = TodoItem::find()
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|b| b.name)
        .collect::<Vec<String>>();
    Ok(Json(todo_items))
}

// #[get("/todoitme/<id>")]
// pub async fn get_todo_item_id(
//     db: &State<DatabaseConnection>,
//     id: i64,
// ) -> Result<Json<String>, ErrorResponder> {
//     let db = db as &DatabaseConnection;
//
// }
