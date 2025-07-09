pub use sea_orm_migration::prelude::*;
pub mod m20220101_000001_create_product_table;
pub mod m20240731_000002_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_product_table::Migration),
            Box::new(m20240731_000002_create_users_table::Migration),
        ]
    }
}
