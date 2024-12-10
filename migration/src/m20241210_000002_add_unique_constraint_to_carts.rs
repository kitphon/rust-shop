use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("unique_product_customer")
                    .table(Carts::Table)
                    .col(Carts::ProductId)
                    .col(Carts::CustomerId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("unique_product_customer")
                    .table(Carts::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
pub enum Carts {
    Table,
    ProductId,
    CustomerId,
}
