#![allow(dead_code)]

use sea_query::{PostgresQueryBuilder, Query, ReturningClause};
use sea_query_binder::SqlxBinder;
use sqlx_queries::{create_connections_pool, Posts, User, Users};

#[tokio::main]
async fn main() {
    insert_many().await;
}

async fn insert_one() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::insert()
        .into_table(Users::Table)
        .columns([Users::FirstName, Users::LastName])
        .values(["John".into(), "Doe".into()])
        .unwrap()
        .returning(ReturningClause::All)
        .build_sqlx(PostgresQueryBuilder);

    let user: User = sqlx::query_as_with(&sql, values)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{user:#?}");
}

async fn insert_many() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::insert()
        .into_table(Users::Table)
        .columns([Users::FirstName, Users::LastName])
        .values(["John".into(), "Doe".into()])
        .unwrap()
        .values(["Will".into(), "Smith".into()])
        .unwrap()
        .values(["Jack".into(), "Sparrow".into()])
        .unwrap()
        .returning(ReturningClause::All)
        .build_sqlx(PostgresQueryBuilder);
    let users: Vec<User> = sqlx::query_as_with(&sql, values)
        .fetch_all(&pool)
        .await
        .unwrap();

    let john = users.get(0).unwrap();
    let (sql, values) = Query::insert()
        .into_table(Posts::Table)
        .columns([Posts::Title, Posts::Content, Posts::UserId])
        .values_panic(["The Rise of Artificial Intelligence".into(), "Artificial intelligence (AI) is revolutionizing industries worldwide. From healthcare innovations to autonomous vehicles, AI is reshaping how we interact with technology.".into(), john.id.into()])
        .values_panic(["The Benefits of Mindful Meditation".into(), "Mindful meditation is a practice that encourages individuals to focus on the present moment. Studies show it can reduce stress, enhance mental clarity, and improve emotional well-being.".into(), john.id.into()])
        .values_panic(["Exploring the Depths of the Ocean".into(), "The ocean covers more than 70% of our planet, yet much of it remains unexplored. Recent advancements in marine technology are unveiling secrets of the deep, from vibrant ecosystems to previously unknown species.".into(), john.id.into()])
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&pool).await.unwrap();

    let will = users.get(1).unwrap();
    let (sql, values) = Query::insert()
        .into_table(Posts::Table)
        .columns([Posts::Title, Posts::Content, Posts::UserId])
        .values_panic(["The Future of Renewable Energy".into(), "Solar, wind, and other renewable sources are becoming increasingly affordable and efficient. These advancements bring hope for a sustainable future while reducing global reliance on fossil fuels.".into(), will.id.into()])
        .values_panic(["The Art of Minimalist Living".into(), "Minimalist living focuses on owning less to create space for what truly matters. It promotes mental clarity and helps reduce environmental impact.".into(), will.id.into()])
        .values_panic(["Why Sleep is Your Superpower".into(), "Quality sleep is essential for memory, focus, and overall health. Prioritizing rest can dramatically improve your physical and mental performance.".into(), will.id.into()])
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&pool).await.unwrap();

    let jack = users.get(2).unwrap();
    let (sql, values) = Query::insert()
        .into_table(Posts::Table)
        .columns([Posts::Title, Posts::Content, Posts::UserId])
        .values_panic(["The Impact of Social Media on Society".into(), "Social media connects people globally but also raises concerns about misinformation and mental health. Striking a balance between online and offline interactions is key.".into(), jack.id.into()])
        .values_panic(["The Mystery of Black Holes".into(), "Black holes are fascinating cosmic phenomena with gravitational pulls so strong that not even light can escape. They continue to be a source of intrigue for astronomers and physicists alike.".into(), jack.id.into()])
        .values_panic(["The Joy of Home Gardening".into(), "Home gardening not only beautifies your space but also promotes sustainability and relaxation. Growing your own herbs and vegetables can be both fulfilling and cost-effective.".into(), jack.id.into()])
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&pool).await.unwrap();
}
