pub use sea_orm_migration::prelude::*;

mod m20241122_085421_create_users;
mod m20241122_125531_create_posts;
mod m20241122_130446_create_comments;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241122_085421_create_users::Migration),
            Box::new(m20241122_125531_create_posts::Migration),
            Box::new(m20241122_130446_create_comments::Migration),
        ]
    }
}
