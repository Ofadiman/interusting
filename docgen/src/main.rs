use convert_case::{Case, Casing};
use std::{
    env,
    fs::{self},
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    description: String,
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

    let mut cargos: Vec<CargoToml> = vec![];

    for project in projects {
        let path = "../".to_string() + &project + "/Cargo.toml";

        let stringified = fs::read_to_string(path).expect("reading Cargo.toml must not fail");

        let cargo: CargoToml =
            toml::from_str(&stringified).expect("parsing Cargo.toml string must not fail");

        cargos.push(cargo);
    }

    cargos.sort_by(|left, right| left.package.name.cmp(&right.package.name));

    let mut readme = String::from(
        "# Interusting

The repository contains mini projects implemented in the Rust programming language.

## Projects

",
    );

    for cargo in cargos {
        readme.push_str(&format!(
            "- [{}](/{}) - {}\n",
            cargo.package.name.to_case(Case::Title),
            cargo.package.name,
            cargo.package.description
        ));
    }

    fs::write("../readme.md", readme).expect("regenerating readme must not fail");
}
