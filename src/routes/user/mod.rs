use rocket::{Build, Rocket, routes};

use crate::mounter::Mounter;
mod delete;
mod dto;
mod get;
mod post;
pub struct UserMounter;

impl Mounter for UserMounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build> {
        r.mount(
            "/user",
            routes![get::all, get::by_id, post::single, delete::by_id],
        )
    }
}
