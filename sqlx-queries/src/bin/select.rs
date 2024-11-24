#![allow(dead_code)]

use std::str::FromStr;

use sea_query::{Asterisk, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::types::Uuid;
use sqlx_queries::{create_connections_pool, User, Users};

#[tokio::main]
async fn main() {
    select_many().await;
}

async fn select_one() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::select()
        .columns([Asterisk])
        .from(Users::Table)
        .and_where(
            Expr::col(Users::Id)
                .eq(Uuid::from_str("8159ec2c-aed7-4676-a95c-036a3ea7f1d0").unwrap()),
        )
        .build_sqlx(PostgresQueryBuilder);

    let user: User = sqlx::query_as_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{user:#?}")
}

async fn select_many() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::select()
        .columns([Asterisk])
        .from(Users::Table)
        .build_sqlx(PostgresQueryBuilder);

    let users: Vec<User> = sqlx::query_as_with(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{users:#?}")
}
