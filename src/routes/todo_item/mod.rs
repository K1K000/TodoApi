use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;

pub mod get;

pub struct TodoItemMounter;

impl Mounter for TodoItemMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount("/todoitem", routes![get::all, get::by_id])
    }
}
