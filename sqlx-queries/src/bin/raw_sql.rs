#![allow(dead_code)]

use sqlx::{
    prelude::FromRow,
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
};
use sqlx_queries::create_connections_pool;

#[derive(FromRow, Debug)]
struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[tokio::main]
async fn main() {
    let pool = create_connections_pool().await;

    let john: User =
        sqlx::query_as("insert into users (first_name, last_name) values ($1, $2) returning *;")
            .bind("John".to_string())
            .bind("Doe".to_string())
            .fetch_one(&pool)
            .await
            .unwrap();
    println!("{john:?}");

    let updated_john: User = sqlx::query_as(
        "update users set last_name = $1, updated_at = $2 where id = $3 returning *;",
    )
    .bind("Smith")
    .bind(Utc::now())
    .bind(john.id)
    .fetch_one(&pool)
    .await
    .unwrap();
    println!("{updated_john:?}");

    let users: Vec<User> = sqlx::query_as("select * from users offset 0 limit 5;")
        .fetch_all(&pool)
        .await
        .unwrap();
    println!("{users:?}");

    let deleted_john: User = sqlx::query_as("delete from users where id = $1 returning *;")
        .bind(updated_john.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{deleted_john:?}")
}
