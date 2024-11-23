use chrono::Utc;
use sea_orm::ActiveModelTrait;
use sea_orm::{
    prelude::Uuid, AccessMode, DatabaseConnection, DbErr, EntityTrait, IsolationLevel, Set,
    TransactionTrait,
};
use sea_orm_queries::entities::users;
use sea_orm_queries::get_database;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let database = get_database().await;

    transactions(&database).await;
}

async fn transactions(database: &DatabaseConnection) {
    let _ = database
        .transaction_with_config::<_, (), DbErr>(
            |tx| {
                Box::pin(async move {
                    let mut john: users::ActiveModel = users::Entity::find_by_id(
                        Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad").unwrap(),
                    )
                    .one(tx)
                    .await
                    .unwrap()
                    .unwrap()
                    .into();

                    let mut jack: users::ActiveModel = users::Entity::find_by_id(
                        Uuid::from_str("01ca873d-db15-4d9c-a213-065309b550d0").unwrap(),
                    )
                    .one(tx)
                    .await
                    .unwrap()
                    .unwrap()
                    .into();

                    let now = Utc::now().fixed_offset();

                    john.updated_at = Set(now);
                    jack.updated_at = Set(now);

                    john.save(tx).await?;

                    // Rollback transaction after error is returned from closure.
                    if true {
                        return Err(DbErr::Query(sea_orm::RuntimeErr::Internal(
                            "Force Rollback!".to_owned(),
                        )));
                    }

                    jack.save(tx).await?;

                    Ok(())
                })
            },
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await;
}
