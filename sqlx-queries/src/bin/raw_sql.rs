use sqlx_queries::create_connections_pool;

#[tokio::main]
async fn main() {
    let pool = create_connections_pool().await;

    let row: (i64,) = sqlx::query_as("select $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{row:?}");
}
