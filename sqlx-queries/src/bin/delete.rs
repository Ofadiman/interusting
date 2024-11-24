use std::str::FromStr;

use sea_query::{Expr, PostgresQueryBuilder, Query, ReturningClause};
use sea_query_binder::SqlxBinder;
use sqlx::types::Uuid;
use sqlx_queries::{create_connections_pool, User, Users};

#[tokio::main]
async fn main() {
    delete_one().await;
}

async fn delete_one() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::delete()
        .from_table(Users::Table)
        .and_where(
            Expr::col(Users::Id)
                .eq(Uuid::from_str("8159ec2c-aed7-4676-a95c-036a3ea7f1d0").unwrap()),
        )
        .returning(ReturningClause::All)
        .build_sqlx(PostgresQueryBuilder);

    let deleted_user: User = sqlx::query_as_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{deleted_user:#?}")
}
