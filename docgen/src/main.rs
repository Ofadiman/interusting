use std::{
    env,
    fs::{self},
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
}

#[derive(Deserialize, Debug)]
struct CargoToml {
    package: Package,
}

fn main() {
    let current_dir = env::current_dir().expect("reading current dir must not fail");
    let root_dir = current_dir
        .parent()
        .expect("getting parent dir must not fail");

    let mut projects: Vec<String> = vec![];

    for dir_entry_result in fs::read_dir(root_dir).expect("reading root dir must not fail") {
        if let Ok(dir_entry) = dir_entry_result {
            if dir_entry
                .metadata()
                .expect("getting file metadata must not fail")
                .is_dir()
            {
                let file_name = dir_entry
                    .file_name()
                    .into_string()
                    .expect("parsing os string into string must not fail");

                if !file_name.contains(".git") {
                    projects.push(file_name);
                }
            }
        }
    }

    for project in projects {
        let path = "../".to_string() + &project + "/Cargo.toml";

        let stringified = fs::read_to_string(path).expect("reading Cargo.toml must not fail");

        let c: CargoToml =
            toml::from_str(&stringified).expect("parsing Cargo.toml string must not fail");

        println!("{c:#?}");
    }
}
