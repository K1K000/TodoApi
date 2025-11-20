use rocket::serde::{Deserialize, Serialize};

#[derive(serde::Serialize)]
pub struct TodoItemResponse {
    pub id: i32,
    pub name: String,
    pub is_complete: bool,
}

// impl Model to TodoItemResponse later

#[derive(Deserialize)]
pub struct CreateTodoItem {
    pub name: String,
    pub is_complete: bool,
}
