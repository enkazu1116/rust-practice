use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261011_071042_create_table_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Discriminator)
                            .varchar(24)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Name)
                            .varchar(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .varchar(255)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        manager
        .drop_table(Table::drop().table(Users::Table).to_owned())
        .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Discriminator,
    Name,
    Email,
}