pub use sea_orm_migration::prelude::*;

mod m01_create_todo_table;
mod m02_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m01_create_todo_table::Migration)];
        vec![Box::new(m02_create_user_table::Migration)]
    }
}
