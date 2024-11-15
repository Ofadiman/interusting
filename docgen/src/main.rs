use std::{
    env,
    fs::{self},
};

fn main() {
    let current_dir = env::current_dir().expect("reading current dir must not fail");
    let root_dir = current_dir
        .parent()
        .expect("getting parent dir must not fail");

    let mut projects: Vec<String> = vec![];
    let root_dir_paths = fs::read_dir(root_dir).expect("reading root dir must not fail");

    for dir_entry_result in root_dir_paths {
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

    println!("{projects:#?}");
}
