use rand::{seq::IteratorRandom, thread_rng};

fn main() {
    let mut rng = thread_rng();

    let programming_languages = vec![
        "Python".to_string(),
        "JavaScript".to_string(),
        "Java".to_string(),
        "C#".to_string(),
        "C++".to_string(),
        "Rust".to_string(),
        "Go".to_string(),
        "Kotlin".to_string(),
        "Swift".to_string(),
        "TypeScript".to_string(),
    ];

    let selected_programming_languages = programming_languages.iter().choose_multiple(&mut rng, 3);

    println!("{selected_programming_languages:#?}");
}
