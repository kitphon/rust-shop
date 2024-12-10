pub use sea_orm_migration::prelude::*;

mod m20241205_000001_create_products_table;
mod m20241205_000002_add_products_to_table;
mod m20241206_000001_create_customers_table;
mod m20241210_000001_create_carts_table;
mod m20241210_000002_add_unique_constraint_to_carts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241205_000001_create_products_table::Migration),
            Box::new(m20241205_000002_add_products_to_table::Migration),
            Box::new(m20241206_000001_create_customers_table::Migration),
            Box::new(m20241210_000001_create_carts_table::Migration),
            Box::new(m20241210_000002_add_unique_constraint_to_carts::Migration),
        ]
    }
}
