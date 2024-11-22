#![allow(dead_code)]

use sea_orm::prelude::Uuid;
use sea_orm::{entity::*, query::*, DatabaseConnection};
use sea_orm::{EntityTrait, QueryFilter};
use sea_orm_queries::entities::{posts, prelude::*, users};
use sea_orm_queries::get_database;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let database = get_database().await;

    pagination(&database).await;
}

async fn find_one(database: &DatabaseConnection) {
    let john = Users::find_by_id(Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad").unwrap())
        .one(database)
        .await
        .unwrap();

    println!("{john:#?}");
}

async fn find_many(database: &DatabaseConnection) {
    let mut query = Users::find();

    query = query.filter(users::Column::FirstName.eq("John"));
    query = query.filter(users::Column::LastName.eq("Doe"));
    query = query.order_by_asc(users::Column::FirstName);

    let result: Vec<users::Model> = query.all(database).await.unwrap();

    println!("{result:#?}");
}

async fn lazy_loading(database: &DatabaseConnection) {
    let john = Users::find_by_id(Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad").unwrap())
        .one(database)
        .await
        .unwrap()
        .unwrap();

    println!("{john:#?}");

    let posts: Vec<posts::Model> = john.find_related(Posts).all(database).await.unwrap();

    println!("{posts:#?}")
}

async fn eager_loading(database: &DatabaseConnection) {
    let john_with_posts: Vec<(
        sea_orm_queries::entities::users::Model,
        Vec<sea_orm_queries::entities::posts::Model>,
    )> = Users::find()
        .find_with_related(Posts)
        .filter(
            users::Column::Id.eq(Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad").unwrap()),
        )
        .all(database)
        .await
        .unwrap();

    println!("{john_with_posts:#?}");
}

async fn pagination(database: &DatabaseConnection) {
    let paginated_users: Vec<(
        sea_orm_queries::entities::users::Model,
        Vec<sea_orm_queries::entities::posts::Model>,
    )> = Users::find()
        .find_with_related(Posts)
        .order_by(users::Column::Id, Order::Asc)
        .offset(0)
        .limit(3)
        .all(database)
        .await
        .unwrap();

    println!("{paginated_users:#?}")
}
