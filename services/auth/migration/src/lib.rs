pub use sea_orm_migration::prelude::*;

use crate::migrations::{*};


pub  mod  schemas;
pub mod migrations;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260809_193548_seed_auth_default::Migration)
        ]
    }
}
