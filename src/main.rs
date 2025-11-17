use entities::{prelude::*, *};
use rocket::serde::json::Json;
use rocket::*;
// use sea_orm::sqlx::Database;
use sea_orm::*;
mod entities;
mod errorhand;
mod todo_item;
use crate::todo_item::get_todo_items;

const DATABASE_URL: &str = "sqlite:./sqlite.db?mode=rwc";
pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    Database::connect(DATABASE_URL).await
}

#[launch]
async fn rocket() -> _ {
    let db = match set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };
    rocket::build()
        .manage(db)
        .mount("/", routes![greet, get_todo_items])
}

#[get("/")]
async fn greet() -> &'static str {
    "Haii bitches"
}
