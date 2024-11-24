#![allow(dead_code)]

use std::collections::HashMap;

use sea_query::{Alias, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    Row,
};
use sqlx_queries::{create_connections_pool, Posts, Users};

#[tokio::main]
async fn main() {
    joins().await;
}

#[derive(Debug)]
struct Post {
    id: Uuid,
    title: String,
    content: String,
    user_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
struct User {
    id: Uuid,
    first_name: String,
    last_name: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    posts: Vec<Post>,
}

async fn joins() {
    let pool = create_connections_pool().await;

    let (sql, values) = Query::select()
        .expr_as(Expr::col((Users::Table, Users::Id)), Alias::new("users_id"))
        .expr_as(
            Expr::col((Users::Table, Users::FirstName)),
            Alias::new("users_first_name"),
        )
        .expr_as(
            Expr::col((Users::Table, Users::LastName)),
            Alias::new("users_last_name"),
        )
        .expr_as(
            Expr::col((Users::Table, Users::CreatedAt)),
            Alias::new("users_created_at"),
        )
        .expr_as(
            Expr::col((Users::Table, Users::UpdatedAt)),
            Alias::new("users_updated_at"),
        )
        .expr_as(Expr::col((Posts::Table, Posts::Id)), Alias::new("posts_id"))
        .expr_as(
            Expr::col((Posts::Table, Posts::Title)),
            Alias::new("posts_title"),
        )
        .expr_as(
            Expr::col((Posts::Table, Posts::Content)),
            Alias::new("posts_content"),
        )
        .expr_as(
            Expr::col((Posts::Table, Posts::CreatedAt)),
            Alias::new("posts_created_at"),
        )
        .expr_as(
            Expr::col((Posts::Table, Posts::UpdatedAt)),
            Alias::new("posts_updated_at"),
        )
        .expr_as(
            Expr::col((Posts::Table, Posts::UserId)),
            Alias::new("posts_user_id"),
        )
        .from(Users::Table)
        .left_join(
            Posts::Table,
            Expr::col((Users::Table, Users::Id)).equals((Posts::Table, Posts::UserId)),
        )
        .build_sqlx(PostgresQueryBuilder);

    println!("{sql}");

    let rows = sqlx::query_with(&sql, values.clone())
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut users: HashMap<Uuid, User> = HashMap::new();

    for row in rows {
        let user_id: Uuid = row.get("users_id");

        let user = users.entry(user_id).or_insert(User {
            id: user_id,
            first_name: row.get("users_first_name"),
            last_name: row.get("users_last_name"),
            created_at: row.get("users_created_at"),
            updated_at: row.get("users_updated_at"),
            posts: Vec::new(),
        });

        if let Some(post_id) = row.try_get::<Uuid, _>("posts_id").ok() {
            user.posts.push(Post {
                id: post_id,
                user_id: row.get("posts_user_id"),
                title: row.get("posts_title"),
                content: row.get("posts_content"),
                created_at: row.get("posts_created_at"),
                updated_at: row.get("posts_updated_at"),
            });
        }
    }

    let users: Vec<User> = users.into_values().collect();

    println!("{users:#?}");
}
