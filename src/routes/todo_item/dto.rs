use rocket::serde::{Deserialize, Serialize};

use crate::entities::todo_item;

#[derive(Serialize, Deserialize)]
pub struct ResponseTodoItem {
    pub id: i32,
    pub name: String,
    pub is_complete: bool,
    pub user_id: i32,
}

pub fn todo_item_to_dto(todo_item: todo_item::Model) -> ResponseTodoItem {
    ResponseTodoItem {
        id: todo_item.id,
        name: todo_item.name,
        is_complete: todo_item.is_complete,
        user_id: todo_item.user_id,
    }
}
// impl Model to TodoItemResponse later

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub name: String,
    pub is_complete: bool,
    pub user_id: i32,
}
