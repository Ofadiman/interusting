use rand::distributions::{Distribution, Uniform};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rayon::prelude::*;

static SEED: u64 = 0;
static BATCH_SIZE: u64 = 100;
static BATCHES: u64 = 10;

fn main() {
    let range = Uniform::new(1, 100);

    let result: Vec<u64> = (1..=BATCHES)
        .into_par_iter()
        .map(|i| {
            let mut rng = ChaCha8Rng::seed_from_u64(SEED);

            rng.set_stream(i);

            let mut numbers: Vec<u64> = Vec::with_capacity(BATCH_SIZE as usize);

            for _ in 0..BATCH_SIZE {
                numbers.push(range.sample(&mut rng));
            }

            numbers
        })
        .map(|vec| {
            let reduced = vec
                .into_iter()
                .reduce(|acc, element| acc + element)
                .unwrap();

            reduced
        })
        .collect();

    println!("{result:?}");
}
