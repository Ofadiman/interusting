use rand::{seq::IteratorRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(1);

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
