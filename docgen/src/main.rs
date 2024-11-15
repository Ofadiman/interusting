use std::env;

fn main() {
    let current_dir = env::current_dir().expect("reading current directory must not fail");
    let root_dir = current_dir
        .parent()
        .expect("parent directory must be defined");

    println!("{}", root_dir.display())
}
