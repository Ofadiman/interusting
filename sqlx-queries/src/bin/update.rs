#![allow(dead_code)]

use std::str::FromStr;

use sea_query::{Expr, PostgresQueryBuilder, Query, ReturningClause};
use sea_query_binder::SqlxBinder;
use sqlx::types::{chrono::Utc, Uuid};
use sqlx_queries::{create_connections_pool, User, Users};

#[tokio::main]
async fn main() {
    update_many().await;
}

async fn update_one() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::update()
        .table(Users::Table)
        .value(Users::UpdatedAt, Utc::now())
        .and_where(
            Expr::col(Users::Id)
                .eq(Uuid::from_str("8159ec2c-aed7-4676-a95c-036a3ea7f1d0").unwrap()),
        )
        .returning(ReturningClause::All)
        .build_sqlx(PostgresQueryBuilder);

    let user: User = sqlx::query_as_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{user:#?}")
}

async fn update_many() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::update()
        .table(Users::Table)
        .value(Users::UpdatedAt, Expr::current_timestamp())
        .returning(ReturningClause::All)
        .build_sqlx(PostgresQueryBuilder);

    let users: Vec<User> = sqlx::query_as_with(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{users:#?}")
}
