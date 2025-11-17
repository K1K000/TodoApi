mod entities;
mod errorhand;
mod mounter;
mod routes;

use entities::{prelude::*, *};
use rocket::serde::json::Json;
use rocket::*;
// use sea_orm::sqlx::Database;
use sea_orm::*;

use crate::mounter::RocketMount;
use crate::routes::todo_item::TodoItemMounter;

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
    rocket::build().manage(db).mount_route::<TodoItemMounter>()
}

#[get("/")]
async fn greet() -> &'static str {
    "Haii bitches"
}
