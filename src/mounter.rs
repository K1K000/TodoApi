use rocket::{Build, Rocket};

pub trait RocketMount {
    fn mount_route<T>(self) -> Rocket<Build>
    where
        T: Mounter,
        Self: Into<Rocket<Build>>,
    {
        T::mount(self.into())
    }
}

pub trait Mounter {
    fn mount(r: Rocket<Build>) -> Rocket<Build>;
}

impl RocketMount for Rocket<Build> {}
