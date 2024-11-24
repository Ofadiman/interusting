use sea_query::Iden;
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::prelude::FromRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::Pool;

pub async fn create_connections_pool() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(10)
        .connect("postgres://user:password@localhost:5432/postgres")
        .await
        .unwrap();

    return pool;
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    CreatedAt,
    UpdatedAt,
}

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
