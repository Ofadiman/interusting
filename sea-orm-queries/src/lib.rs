pub mod entities;

use sea_orm::*;
use std::time::Duration;

// https://www.sea-ql.org/SeaORM/docs/install-and-config/debug-log/
fn initialize_debug_log() {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}

pub async fn get_database() -> DatabaseConnection {
    initialize_debug_log();

    let mut options = ConnectOptions::new("postgres://user:password@localhost:5432/postgres");

    options
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .set_schema_search_path("public");

    return Database::connect(options).await.unwrap();
}
