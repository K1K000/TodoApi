use crate::entities::{prelude::User, user};
use crate::errorhand::ErrorResponder;
use crate::routes::user::dto::CreateUser;
// use rocket::serde::Deserialize;
use rocket::{http::Status, serde::json::Json, *};
use sea_orm::{entity::*, query::*, *};

#[put("/<id>", data = "<new_item>", format = "json")]
pub async fn put(
    new_item: Json<CreateUser>,
    id: i32,
    db: &State<DatabaseConnection>,
) -> Result<Status, ErrorResponder> {
    let db = db.inner();
    match User::find_by_id(id).one(db).await? {
        Some(_val) => {
            let new_itemm = user::ActiveModel {
                id: sea_orm::ActiveValue::set(id),
                name: Set(new_item.name.clone()),
            };
            user::Entity::update(new_itemm)
                .filter(user::Column::Id.contains(id.to_string()))
                .exec(db)
                .await?;
            Ok(Status::NoContent)
        }
        None => Err(ErrorResponder::NotFound(())),
    }
}
