use chrono::Utc;
use futures::StreamExt;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm_queries::entities::prelude::*;
use sea_orm_queries::get_database;

#[tokio::main]
async fn main() {
    let database = get_database().await;

    streaming(&database).await;
}

async fn streaming(database: &DatabaseConnection) {
    let mut stream = Users::find().stream(database).await.unwrap();

    while let Some(user) = stream.next().await {
        println!("{:#?}, {:#?}", Utc::now(), user)
    }
}
