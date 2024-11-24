#![allow(dead_code)]

use sea_query::{PostgresQueryBuilder, Query, ReturningClause};
use sea_query_binder::SqlxBinder;
use sqlx_queries::{create_connections_pool, User, Users};

#[tokio::main]
async fn main() {
    insert_many().await;
}

async fn insert_one() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::insert()
        .into_table(Users::Table)
        .columns([Users::FirstName, Users::LastName])
        .values(["John".into(), "Doe".into()])
        .unwrap()
        .returning(ReturningClause::All)
        .build_sqlx(PostgresQueryBuilder);

    let user: User = sqlx::query_as_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{user:#?}");
}

async fn insert_many() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::insert()
        .into_table(Users::Table)
        .columns([Users::FirstName, Users::LastName])
        .values(["John".into(), "Doe".into()])
        .unwrap()
        .values(["Will".into(), "Smith".into()])
        .unwrap()
        .values(["Jack".into(), "Sparrow".into()])
        .unwrap()
        .returning(ReturningClause::All)
        .build_sqlx(PostgresQueryBuilder);

    let rows: Vec<User> = sqlx::query_as_with(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{rows:#?}");
}
