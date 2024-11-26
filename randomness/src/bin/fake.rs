#![allow(dead_code)]

use chrono::{DateTime, Utc};
use fake::Fake;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use uuid::Uuid;

#[derive(Debug)]
struct Person {
    id: Uuid,
    first_name: String,
    last_name: String,
    created_at: DateTime<Utc>,
}

impl Person {
    fn random<R: Rng + ?Sized>(rng: &mut R) -> Person {
        let first_name: String = fake::faker::name::en::FirstName().fake_with_rng(rng);
        let last_name: String = fake::faker::name::en::LastName().fake_with_rng(rng);
        let id: Uuid = fake::uuid::UUIDv4.fake_with_rng(rng);
        let created_at: DateTime<Utc> = fake::faker::chrono::en::DateTimeBetween(
            DateTime::parse_from_rfc3339("2024-01-01T09:00:00.000Z")
                .unwrap()
                .to_utc(),
            DateTime::parse_from_rfc3339("2024-06-30T09:00:00.000Z")
                .unwrap()
                .to_utc(),
        )
        .fake_with_rng(rng);

        Person {
            id,
            first_name,
            last_name,
            created_at,
        }
    }
}

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(0);

    let person = Person::random(&mut rng);

    println!("{person:#?}");
}
