use sqlx::postgres::{PgPoolOptions, Postgres};
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
