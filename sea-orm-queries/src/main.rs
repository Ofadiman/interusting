mod entities;

use entities::bakery;
use entities::{prelude::*, *};
use sea_orm::*;

const DATABASE_URL: &str = "postgres://user:password@localhost:5432/postgres";

#[tokio::main]
async fn main() {
    if let Err(err) = crud().await {
        panic!("{}", err);
    }
}

async fn crud() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Bistro Charlotte".to_string()),
        profit_margin: ActiveValue::Set(8.0),
        ..Default::default()
    };

    // InsertResult { last_insert_id: 1 }
    let insert_result = Bakery::insert(bakery).exec(&db).await?;

    // ActiveModel { id: Unchanged(1), name: Unchanged("Bistro Charlotte"), profit_margin: Unchanged(8.0) }
    let mut bakery: bakery::ActiveModel = Bakery::find_by_id(insert_result.last_insert_id)
        .one(&db)
        .await?
        .unwrap()
        .into();

    bakery.name = ActiveValue::Set("Lajkonik".to_string());
    bakery.update(&db).await?;

    let john = chef::ActiveModel {
        name: ActiveValue::Set("John".to_string()),
        bakery_id: ActiveValue::Set(insert_result.last_insert_id),
        ..Default::default()
    };
    Chef::insert(john).exec(&db).await?;

    // [Model { id: 1, name: "Lajkonik", profit_margin: 8.0 }]
    let mut bakeries: Vec<bakery::Model> = Bakery::find().all(&db).await?;

    // [Model { id: 1, name: "John", contact_details: None, bakery_id: 4 }]
    let mut chiefs: Vec<chef::Model> = Chef::find().all(&db).await?;

    chiefs.remove(0).delete(&db).await?;
    bakeries.remove(0).delete(&db).await?;

    Ok(())
}
