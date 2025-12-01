use crate::entities::prelude::User;
use crate::errorhand::{ErrorMessage, ErrorResponder};
use crate::routes::user::dto::*;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[get("/")]
pub async fn all(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseUser>>, ErrorResponder> {
    let db = db.inner();
    Ok(Json(
        User::find()
            .all(db)
            .await?
            .iter()
            .map(|val| user_to_dto(val.clone()))
            .collect(),
    ))
}

#[get("/<id>")]
pub async fn by_id(
    db: &State<DatabaseConnection>,
    id: i32,
) -> Result<Json<ResponseUser>, ErrorResponder> {
    let db = db.inner();
    let user = User::find_by_id(id).one(db).await?.ok_or_else(|| {
        ErrorResponder::BadRequest(Json(ErrorMessage {
            message: "Record doesnt exist".into(),
        }))
    })?;
    Ok(Json(user_to_dto(user)))
}
