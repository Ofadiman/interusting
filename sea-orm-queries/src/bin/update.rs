#![allow(dead_code)]

use chrono::Utc;
use sea_orm::entity::*;
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm_queries::entities::users;
use sea_orm_queries::get_database;
use sea_query::Expr;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let database = get_database().await;

    update_many(&database).await;
}

async fn update_one(database: &DatabaseConnection) {
    let mut john: users::ActiveModel =
        users::Entity::find_by_id(Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad").unwrap())
            .one(database)
            .await
            .unwrap()
            .unwrap()
            .into();

    john.updated_at = Set(Utc::now().fixed_offset());

    let updated_john = john.update(database).await.unwrap();
    println!("{updated_john:#?}");
}

async fn update_many(database: &DatabaseConnection) {
    let updated_users: Vec<users::Model> = users::Entity::update_many()
        .col_expr(
            users::Column::UpdatedAt,
            Expr::value(Utc::now().fixed_offset()),
        )
        .filter(users::Column::FirstName.is_in(["Will".to_string(), "John".to_string()]))
        .exec_with_returning(database)
        .await
        .unwrap();

    println!("{updated_users:#?}");
}
