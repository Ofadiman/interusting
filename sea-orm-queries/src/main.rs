#![allow(dead_code)]

mod entities;

use std::time::Duration;

use entities::bakery;
use entities::{prelude::*, *};
use sea_orm::*;
use sea_query::{Alias, Expr, Query};

const DATABASE_URL: &str = "postgres://user:password@localhost:5432/postgres";

#[tokio::main]
async fn main() {
    if let Err(err) = query_builder().await {
        panic!("{}", err);
    }
}

async fn database() -> DatabaseConnection {
    let mut options = ConnectOptions::new("postgres://user:password@localhost:5432/postgres");

    options
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .set_schema_search_path("public");

    return Database::connect(options).await.unwrap();
}

async fn crud() -> Result<(), DbErr> {
    let db = &database().await;

    let bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Bistro Charlotte".to_string()),
        profit_margin: ActiveValue::Set(8.0),
        ..Default::default()
    };

    // InsertResult { last_insert_id: 1 }
    let insert_result = Bakery::insert(bakery).exec(db).await?;

    // ActiveModel { id: Unchanged(1), name: Unchanged("Bistro Charlotte"), profit_margin: Unchanged(8.0) }
    let mut bakery: bakery::ActiveModel = Bakery::find_by_id(insert_result.last_insert_id)
        .one(db)
        .await?
        .unwrap()
        .into();

    bakery.name = ActiveValue::Set("Lajkonik".to_string());
    bakery.update(db).await?;

    let john = chef::ActiveModel {
        name: ActiveValue::Set("John".to_string()),
        bakery_id: ActiveValue::Set(insert_result.last_insert_id),
        ..Default::default()
    };
    Chef::insert(john).exec(db).await?;

    // [Model { id: 1, name: "Lajkonik", profit_margin: 8.0 }]
    let mut bakeries: Vec<bakery::Model> = Bakery::find().all(db).await?;

    // [Model { id: 1, name: "John", contact_details: None, bakery_id: 4 }]
    let mut chefs: Vec<chef::Model> = Chef::find().all(db).await?;

    chefs.remove(0).delete(db).await?;
    bakeries.remove(0).delete(db).await?;

    Ok(())
}

async fn relationships() -> Result<(), DbErr> {
    let db = &database().await;

    let la_boulangerie = bakery::ActiveModel {
        name: ActiveValue::Set("La Boulangerie".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(la_boulangerie).exec(db).await?;
    for chef_name in ["Jolie", "Charles", "Madeleine", "Frederic"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    let la_id = bakery_res.last_insert_id;

    let arte_by_padaria = bakery::ActiveModel {
        name: ActiveValue::Set("Arte by Padaria".to_owned()),
        profit_margin: ActiveValue::Set(0.2),
        ..Default::default()
    };
    let bakery_res = Bakery::insert(arte_by_padaria).exec(db).await?;
    for chef_name in ["Brian", "Charles", "Kate", "Samantha"] {
        let chef = chef::ActiveModel {
            name: ActiveValue::Set(chef_name.to_owned()),
            bakery_id: ActiveValue::Set(bakery_res.last_insert_id),
            ..Default::default()
        };
        Chef::insert(chef).exec(db).await?;
    }
    let arte_id = bakery_res.last_insert_id;

    // [
    //     Model {
    //         id: 1,
    //         name: "La Boulangerie",
    //         profit_margin: 0.0,
    //     },
    //     Model {
    //         id: 2,
    //         name: "Arte by Padaria",
    //         profit_margin: 0.2,
    //     },
    // ]
    let bakeries: Vec<bakery::Model> = Bakery::find()
        .filter(
            Condition::any()
                .add(bakery::Column::Id.eq(la_id))
                .add(bakery::Column::Id.eq(arte_id)),
        )
        .all(db)
        .await?;

    // [
    //     [
    //         Model {
    //             id: 1,
    //             name: "Jolie",
    //             contact_details: None,
    //             bakery_id: 1,
    //         },
    //         ...,
    //         Model {
    //             id: 2,
    //             name: "Frederic",
    //             contact_details: None,
    //             bakery_id: 1,
    //         },
    //     ],
    //     [
    //         Model {
    //             id: 5,
    //             name: "Brian",
    //             contact_details: None,
    //             bakery_id: 2,
    //         },
    //         ...,
    //         Model {
    //             id: 6,
    //             name: "Samantha",
    //             contact_details: None,
    //             bakery_id: 2,
    //         },
    //     ],
    // ]
    let chefs: Vec<Vec<chef::Model>> = bakeries.load_many(Chef, db).await?;

    let mut la_chef_names: Vec<String> = chefs[0].to_owned().into_iter().map(|b| b.name).collect();
    la_chef_names.sort_unstable();
    assert_eq!(la_chef_names, ["Charles", "Frederic", "Jolie", "Madeleine"]);

    let mut arte_chef_names: Vec<String> =
        chefs[1].to_owned().into_iter().map(|b| b.name).collect();
    arte_chef_names.sort_unstable();
    assert_eq!(arte_chef_names, ["Brian", "Charles", "Kate", "Samantha"]);

    Ok(())
}

#[derive(FromQueryResult)]
struct ChefNameResult {
    name: String,
}

async fn query_builder() -> Result<(), DbErr> {
    let db = &database().await;

    let columns: Vec<Alias> = ["name", "profit_margin"]
        .into_iter()
        .map(Alias::new)
        .collect();

    let mut stmt = Query::insert();
    stmt.into_table(bakery::Entity).columns(columns);

    stmt.values_panic(["SQL Bakery".into(), (-100.0).into()]);

    let builder = db.get_database_backend();
    db.execute(builder.build(&stmt)).await?;

    let column = (chef::Entity, Alias::new("name"));

    let mut stmt = Query::select();
    stmt.column(column.clone())
        .from(chef::Entity)
        .join(
            JoinType::Join,
            bakery::Entity,
            Expr::col((chef::Entity, Alias::new("bakery_id")))
                .equals((bakery::Entity, Alias::new("id"))),
        )
        .order_by(column, Order::Asc);

    let builder = db.get_database_backend();
    let chef = ChefNameResult::find_by_statement(builder.build(&stmt))
        .all(db)
        .await?;
    let _chef_names = chef.into_iter().map(|b| b.name).collect::<Vec<_>>();

    Ok(())
}
