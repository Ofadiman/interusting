use rand::{thread_rng, Rng};
use std::collections::HashMap;

fn main() {
    let mut rng = thread_rng();
    let distribution = rand::distributions::Uniform::new_inclusive(1, 25);

    let mut numbers: HashMap<i32, i32> = HashMap::new();
    for _ in 0..1000 {
        numbers
            .entry(rng.sample(distribution))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut numbers: Vec<(&i32, &i32)> = numbers.iter().collect();
    numbers.sort_by(|(n1, _), (n2, _)| {
        return n1.cmp(n2);
    });

    for (_, count) in numbers {
        println!("{}", "#".repeat(*count as usize))
    }
}
