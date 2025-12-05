use crate::entities::prelude::User;
use crate::entities::user;
use crate::errorhand::ErrorResponder;
use crate::routes::user::dto::*;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[post("/", format = "json", data = "<new_item>")]
pub async fn single(
    db: &State<DatabaseConnection>,
    new_item: Json<CreateUser>,
) -> Result<status::Created<Json<ResponseUser>>, ErrorResponder> {
    let db = db.inner();

    let new_row = user::ActiveModel {
        name: Set(new_item.name.clone()),
        ..Default::default()
    };
    let insert_res = User::insert(new_row).exec(db).await?;
    Ok(
        status::Created::new("http://127.0.0.1:8000/user").body(Json(ResponseUser {
            id: insert_res.last_insert_id,
            name: new_item.name.clone(),
        })),
    )
}
