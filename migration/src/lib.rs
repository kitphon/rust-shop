pub use sea_orm_migration::prelude::*;

mod m20241205_000001_create_products_table;
mod m20241205_000002_add_products_to_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241205_000001_create_products_table::Migration),
            Box::new(m20241205_000002_add_products_to_table::Migration)
        ]
    }
}
