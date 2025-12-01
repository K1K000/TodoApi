use rocket::serde::{Deserialize, Serialize};

use crate::entities::user;

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: i32,
    pub name: String,
}

pub fn user_to_dto(user: user::Model) -> ResponseUser {
    ResponseUser {
        id: user.id,
        name: user.name,
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}
