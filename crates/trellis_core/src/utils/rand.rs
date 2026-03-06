use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn psuedo_random_u64() -> u64 {
    let x = COUNTER.fetch_add(1, Ordering::Relaxed);
    splitmix64(x)
}

// @ref(https://rosettacode.org/wiki/Pseudo-random_numbers/Splitmix64)
//   Stateless paraphrase of the sources Rust example.
fn splitmix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
