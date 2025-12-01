use crate::entities::prelude::User;
use crate::entities::user;
use crate::errorhand::ErrorResponder;
use crate::routes::user::dto::*;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[post("/", format = "json", data = "<new_item>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    new_item: Json<CreateUser>,
) -> Result<&'static str, ErrorResponder> {
    let db = db.inner();

    let new_row = user::ActiveModel {
        name: Set(new_item.name.clone()),
        ..Default::default()
    };
    User::insert(new_row).exec(db).await?;
    Ok("did do it this time")
}
