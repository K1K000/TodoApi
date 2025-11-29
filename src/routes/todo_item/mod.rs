use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod delete;
pub mod dto;
pub mod get;
pub mod post;
pub mod put;

pub struct TodoItemMounter;

impl Mounter for TodoItemMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/todoitem",
            routes![get::all, get::by_id, post::single, put::put, delete::by_id],
        )
    }
}
