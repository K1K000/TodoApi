use rocket::serde::*;
use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_user_table::User;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "02_create_todo_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TodoItem::Table)
                    .col(
                        ColumnDef::new(TodoItem::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TodoItem::Name).string().not_null())
                    .col(ColumnDef::new(TodoItem::IsComplete).boolean().not_null())
                    .col(ColumnDef::new(TodoItem::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_todoitem_user")
                            .from(TodoItem::Table, TodoItem::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden, Serialize, Deserialize)]
enum TodoItem {
    Table,
    Id,
    Name,
    IsComplete,
    UserId,
}
