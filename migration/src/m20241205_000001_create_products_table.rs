use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Products::Table)
                    .if_not_exists()
                    .col(pk_auto(Products::Id))
                    .col(date_time(Products::CreatedAt))
                    .col(date_time(Products::UpdatedAt))
                    .col(string(Products::Title))
                    .col(string(Products::Detail))
                    .col(decimal(Products::Price))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Products::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    Title,
    Detail,
    Price,
}
