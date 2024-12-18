#![allow(dead_code)]

use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Database {
    url: String,
}

#[derive(Deserialize, Debug)]
struct Actix {
    address: String,
    port: u64,
}

#[derive(Deserialize, Debug)]
struct Redis {
    url: String,
}

#[derive(Deserialize, Debug)]
struct Settings {
    actix: Actix,
    database: Database,
    redis: Redis,
}

fn main() {
    dotenvy::from_filename(".example.env").unwrap();

    let settings = Config::builder()
        .add_source(File::new("configs/database.json", FileFormat::Json))
        .add_source(File::new("configs/actix.yaml", FileFormat::Yaml))
        .add_source(
            Environment::default()
                .prefix("INTERUSTING")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()
        .expect("building settings must not fail")
        .try_deserialize::<Settings>()
        .expect("deserializing settings must not fail");

    println!("{settings:#?}");
}
