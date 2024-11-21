use sea_orm::{Database, DbErr};

const DATABASE_URL: &str = "postgres://user:password@localhost:5432/postgres";

async fn run() -> Result<(), DbErr> {
    let _db = Database::connect(DATABASE_URL).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        panic!("{}", err);
    }
}
