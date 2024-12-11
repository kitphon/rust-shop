use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItems::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrderItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OrderItems::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(OrderItems::OrderId).big_integer().not_null())
                    .col(ColumnDef::new(OrderItems::ProductId).big_integer().not_null())
                    .col(ColumnDef::new(OrderItems::UnitPrice).decimal().not_null())
                    .col(ColumnDef::new(OrderItems::Amount).unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItems::Table, OrderItems::OrderId)
                            .to(Orders::Table, Orders::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum OrderItems {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    OrderId,
    ProductId,
    UnitPrice,
    Amount,
}

#[derive(Iden)]
pub enum Orders {
    Table,
    Id,
}
