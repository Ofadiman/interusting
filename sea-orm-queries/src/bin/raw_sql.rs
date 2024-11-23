#![allow(dead_code)]

use sea_orm::{
    prelude::Uuid, DatabaseConnection, DbBackend, EntityTrait, FromQueryResult, Statement,
};
use sea_orm_queries::{entities::users, get_database};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let database = get_database().await;

    select_custom_model(&database).await;
}

async fn select_model(database: &DatabaseConnection) {
    let john: users::Model = users::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select * from users where users.id = $1;"#,
            [Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad")
                .unwrap()
                .into()],
        ))
        .one(database)
        .await
        .unwrap()
        .unwrap();

    println!("{john:#?}");
}

async fn select_custom_model(database: &DatabaseConnection) {
    #[derive(Debug, FromQueryResult)]
    struct PostsByUser {
        user_id: Uuid,
        total: i32,
    }

    let posts_by_user_count: Vec<PostsByUser> =
        PostsByUser::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select user_id, count(*)::int4 as total from posts group by posts.user_id;"#,
            [],
        ))
        .all(database)
        .await
        .unwrap();

    println!("{posts_by_user_count:#?}");
}
