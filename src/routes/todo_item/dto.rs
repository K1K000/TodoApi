use crate::entities::user;
use rocket::serde::{Deserialize, Serialize};

use crate::entities::todo_item;

// thiserror

#[derive(Serialize, Deserialize)]
pub struct ResponseTodoItem {
    pub id: i32,
    pub name: String,
    pub is_complete: bool,
    pub user_id: i32,
    pub user_name: String,
}

// impl TryFrom<todo_item::Model> for ResponseTodoItem {
//     type Error = String;
//
//     // TOD: implement after giving todo_item the new HasOne relation field
//     // O: cant do because 1.1 :(
//     fn try_from(item: todo_item::Model) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: todo!(),
//             name: todo!(),
//             is_complete: todo!(),
//             user_id: todo!(),
//             user_name: item.user.name,
//         })
//     }
// }

// TODO: should be a From<TodoItem>
pub fn todo_item_to_dto(todo_item: &todo_item::Model, user: &user::Model) -> ResponseTodoItem {
    ResponseTodoItem {
        id: todo_item.id,
        name: todo_item.name.to_owned(),
        is_complete: todo_item.is_complete,
        user_id: todo_item.user_id,
        user_name: user.name.to_owned(),
    }
}

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub name: String,
    pub is_complete: bool,
    pub user_id: i32,
}
