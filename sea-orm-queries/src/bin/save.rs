use chrono::Utc;
use sea_orm::prelude::Uuid;
use sea_orm::ActiveModelTrait;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use sea_orm_queries::entities::prelude::*;
use sea_orm_queries::{entities::users, get_database};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let database = get_database().await;

    save(&database).await;
}

async fn save(database: &DatabaseConnection) {
    let mut john: users::ActiveModel =
        Users::find_by_id(Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad").unwrap())
            .one(database)
            .await
            .unwrap()
            .unwrap()
            .into();

    john.updated_at = Set(Utc::now().fixed_offset());

    let updated_john = john.save(database).await.unwrap();

    println!("{updated_john:#?}");
}
