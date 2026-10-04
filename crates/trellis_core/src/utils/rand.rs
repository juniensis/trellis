use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static COUNTER: AtomicU64 = AtomicU64::new(1);

pub fn psuedo_random_u64() -> u64 {
    prand_u64()
}

pub fn psuedo_random_u32() -> u32 {
    prand_u64() as u32
}

pub fn prand_u64() -> u64 {
    let systime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let x = COUNTER.fetch_add(1, Ordering::Relaxed);
    let hi = (systime >> 64) as u64 + x;
    let lo = (systime as u64).wrapping_add(x);
    splitmix64(hi.wrapping_mul(lo))
}

// @ref(https://rosettacode.org/wiki/Pseudo-random_numbers/Splitmix64)
//   Stateless paraphrase of the sources Rust example.
fn splitmix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
