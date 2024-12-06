use sea_orm::sqlx::types::chrono::Utc;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Products::Table)
            .columns([Products::CreatedAt, Products::UpdatedAt, Products::Title, Products::Detail, Products::Price])
            .values_panic([Utc::now().into(), Utc::now().into(), "Pepsi".into(), "Pepsi เครื่องดื่มมีน้ำตาล".into(), 25.into()])
            .values_panic([Utc::now().into(), Utc::now().into(), "Pepsi Max".into(), "Pepsi Max เครื่องดื่มไม่มีน้ำตาล".into(), 25.into()])
            .values_panic([Utc::now().into(), Utc::now().into(), "Coke".into(), "Coke เครื่องดื่มมีน้ำตาล".into(), 24.into()])
            .values_panic([Utc::now().into(), Utc::now().into(), "Coke Zero".into(), "Coke Zero เครื่องดื่มไม่มีน้ำตาล".into(), 24.into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Products::Table)
            .and_where(Expr::col(Products::Title).is_in(vec![
                "Pepsi", "Pepsi Max", "Coke", "Coke Zero",
            ]))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Products {
    Table,
    CreatedAt,
    UpdatedAt,
    Title,
    Detail,
    Price,
}
