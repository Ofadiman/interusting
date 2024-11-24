#![allow(dead_code)]

use std::str::FromStr;

use sea_query::{Asterisk, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::types::Uuid;
use sqlx_queries::{create_connections_pool, User, Users};

#[tokio::main]
async fn main() {
    paginate().await;
}

async fn paginate() {
    let pool = create_connections_pool().await;

    let mut builder = Query::select();

    builder.columns([Asterisk]).from(Users::Table);

    if true {
        builder.and_where(Expr::col(Users::Id).is_in([
            Uuid::from_str("566295d1-c4ff-4cfa-aa35-9ef3a2ac121a").unwrap(),
            Uuid::from_str("1d4205ab-348e-48a0-a8e6-de7dfc80f75f").unwrap(),
            Uuid::from_str("8159ec2c-aed7-4676-a95c-036a3ea7f1d0").unwrap(),
        ]));
    }

    if true {
        builder.offset(0);
    }

    if true {
        builder.limit(50);
    }

    let (sql, values) = builder.build_sqlx(PostgresQueryBuilder);

    let users: Vec<User> = sqlx::query_as_with(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{users:#?}")
}
