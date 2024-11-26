use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    let numbers: [usize; 100] = core::array::from_fn(|i| i + 1);

    let mut selected: Vec<usize> = Vec::with_capacity(10);
    for _ in 0..10 {
        selected.push(numbers.choose(&mut rng).unwrap().clone());
    }

    println!("{selected:?}")
}
