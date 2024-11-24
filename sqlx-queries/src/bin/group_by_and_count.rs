#![allow(dead_code)]

use sea_query::{Alias, Expr, Func, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::prelude::FromRow;
use sqlx_queries::{create_connections_pool, Users};

#[tokio::main]
async fn main() {
    group_by_and_count().await;
}

#[derive(FromRow, Debug)]
struct Row {
    first_name: String,
    last_name: String,
    total: i64,
}

async fn group_by_and_count() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::select()
        .columns([Users::FirstName, Users::LastName])
        .from(Users::Table)
        .expr_as(Func::count(Expr::col(Users::Id)), Alias::new("total"))
        .group_by_columns([Users::FirstName, Users::LastName])
        .build_sqlx(PostgresQueryBuilder);

    let rows: Vec<Row> = sqlx::query_as_with(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{rows:#?}");
}
