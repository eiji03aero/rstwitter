pub use sea_orm_migration::prelude::*;

mod m20230225_145228_create_user;
mod m20230402_122231_create_tweet;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230225_145228_create_user::Migration),
            Box::new(m20230402_122231_create_tweet::Migration),
        ]
    }
}
