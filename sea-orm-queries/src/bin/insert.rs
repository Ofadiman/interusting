use std::str::FromStr;

use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue::{self};
use sea_orm::EntityTrait;
use sea_orm_queries::entities::{posts, prelude::*, users};
use sea_orm_queries::get_database;

#[tokio::main]
async fn main() {
    let database = get_database().await;

    let john_id = Uuid::from_str("4f0a6870-7bcf-4ae2-aa6f-3ce0ae4609ad").unwrap();
    let john = users::ActiveModel {
        id: ActiveValue::Set(john_id),
        first_name: ActiveValue::Set("John".to_string()),
        last_name: ActiveValue::Set("Doe".to_string()),
        ..Default::default()
    };

    let will_id = Uuid::from_str("6bd3794a-f07b-41d3-ba6c-d1ea4800708c").unwrap();
    let will = users::ActiveModel {
        id: ActiveValue::Set(will_id),
        first_name: ActiveValue::Set("Will".to_string()),
        last_name: ActiveValue::Set("Smith".to_string()),
        ..Default::default()
    };

    let jack_id = Uuid::from_str("01ca873d-db15-4d9c-a213-065309b550d0").unwrap();
    let jack = users::ActiveModel {
        id: ActiveValue::Set(jack_id),
        first_name: ActiveValue::Set("Jack".to_string()),
        last_name: ActiveValue::Set("Sparrow".to_string()),
        ..Default::default()
    };

    Users::insert_many([john, will, jack])
        .exec(&database)
        .await
        .unwrap();

    let johns_post_1_id = Uuid::from_str("2cf4d21e-149c-4042-97bd-e71499e2c49e").unwrap();
    let johns_post_1 = posts::ActiveModel {
        id: ActiveValue::Set(johns_post_1_id),
        title: ActiveValue::Set("5 Tips for Effective Time Management".to_string()),
        content: ActiveValue::Set("Struggling with time management? Here are five proven strategies to stay productive and stress-free.".to_string()),
        user_id: ActiveValue::Set(john_id),
        ..Default::default()
    };

    let johns_post_2_id = Uuid::from_str("c2f345a8-f8e9-49dd-b295-926ba059c3e5").unwrap();
    let johns_post_2 = posts::ActiveModel {
        id: ActiveValue::Set(johns_post_2_id),
        title: ActiveValue::Set("Why I Switched to a Standing Desk".to_string()),
        content: ActiveValue::Set(
            "After months of back pain, I finally decided to try a standing desk. Here’s what I’ve learned in my first week.".to_string(),
        ),
        user_id: ActiveValue::Set(john_id),
        ..Default::default()
    };

    let johns_post_3_id = Uuid::from_str("ce0a45db-dad5-48a0-8f61-4efdc73cef43").unwrap();
    let johns_post_3 = posts::ActiveModel {
        id: ActiveValue::Set(johns_post_3_id),
        title: ActiveValue::Set("Exploring the Mountains: My Weekend Adventure".to_string()),
        content: ActiveValue::Set(
            "This weekend, I hiked up Mount Willow. The views were breathtaking, and the experience was unforgettable.".to_string(),
        ),
        user_id: ActiveValue::Set(john_id),
        ..Default::default()
    };

    let wills_post_1_id = Uuid::from_str("a0c4f716-993b-4115-a2dc-7bbad6c37b33").unwrap();
    let wills_post_1 = posts::ActiveModel {
        id: ActiveValue::Set(wills_post_1_id),
        title: ActiveValue::Set("Mastering the Art of Bread Baking".to_string()),
        content: ActiveValue::Set(
            "I recently started baking bread at home. Here’s my beginner’s guide to perfect sourdough every time.".to_string(),
        ),
        user_id: ActiveValue::Set(will_id),
        ..Default::default()
    };

    let wills_post_2_id = Uuid::from_str("9f5e9ae6-8c5a-4ab4-bb9a-99857b35d9a1").unwrap();
    let wills_post_2 = posts::ActiveModel {
        id: ActiveValue::Set(wills_post_2_id),
        title: ActiveValue::Set("Top 10 Sci-Fi Books of All Time".to_string()),
        content: ActiveValue::Set(
            "If you’re a sci-fi fan, these ten books are must-reads. From classics to modern masterpieces, this list has it all.".to_string(),
        ),
        user_id: ActiveValue::Set(will_id),
        ..Default::default()
    };

    let wills_post_3_id = Uuid::from_str("6ab7f9de-0139-41a7-900e-17a08e3e83d8").unwrap();
    let wills_post_3 = posts::ActiveModel {
        id: ActiveValue::Set(wills_post_3_id),
        title: ActiveValue::Set("A Guide to Growing Indoor Plants".to_string()),
        content: ActiveValue::Set(
            "Want to make your home greener? Here are simple tips for growing and maintaining indoor plants.".to_string(),
        ),
        user_id: ActiveValue::Set(will_id),
        ..Default::default()
    };

    let jacks_post_1_id = Uuid::from_str("c1f9a8d6-f76a-4d72-9348-22ec2c4b5f8c").unwrap();
    let jacks_post_1 = posts::ActiveModel {
        id: ActiveValue::Set(jacks_post_1_id),
        title: ActiveValue::Set("How I Built My Own Gaming PC".to_string()),
        content: ActiveValue::Set(
            "Building a gaming PC can be intimidating, but it’s easier than you think! Here’s my step-by-step guide.".to_string(),
        ),
        user_id: ActiveValue::Set(jack_id),
        ..Default::default()
    };

    let jacks_post_2_id = Uuid::from_str("7e9c2b8d-4141-4e8f-8a7f-cf3a719e35df").unwrap();
    let jacks_post_2 = posts::ActiveModel {
        id: ActiveValue::Set(jacks_post_2_id),
        title: ActiveValue::Set("10 Hidden Gems on Netflix You Need to Watch".to_string()),
        content: ActiveValue::Set(
            "Tired of scrolling endlessly? Check out these underrated Netflix movies and shows you might have missed.".to_string(),
        ),
        user_id: ActiveValue::Set(jack_id),
        ..Default::default()
    };

    let jacks_post_3_id = Uuid::from_str("86cfb57a-ef3a-46a6-9e1b-56bb3c376b98").unwrap();
    let jacks_post_3 = posts::ActiveModel {
        id: ActiveValue::Set(jacks_post_3_id),
        title: ActiveValue::Set("The Best Coffee Shops in Seattle".to_string()),
        content: ActiveValue::Set(
            "Seattle is famous for its coffee culture. Here’s my list of the top spots to grab a cup of joe.".to_string(),
        ),
        user_id: ActiveValue::Set(jack_id),
        ..Default::default()
    };

    Posts::insert_many([
        jacks_post_1,
        jacks_post_2,
        jacks_post_3,
        johns_post_1,
        johns_post_2,
        johns_post_3,
        wills_post_1,
        wills_post_2,
        wills_post_3,
    ])
    .exec(&database)
    .await
    .unwrap();
}
