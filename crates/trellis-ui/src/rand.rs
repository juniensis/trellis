use std::sync::atomic::{AtomicU64, Ordering};

static STATE: [AtomicU64; 4] = [
    AtomicU64::new(0x243f6a8885a308d3),
    AtomicU64::new(0x13198a2e03707344),
    AtomicU64::new(0xa4093822299f31d0),
    AtomicU64::new(0x082efa98ec4e6c89),
];

fn rotl(x: u64, k: u32) -> u64 {
    x.rotate_left(k)
}

pub fn rand_u128() -> u128 {
    let lo = xoshiro256ss();
    let hi = xoshiro256ss();
    ((hi as u128) << 64) | (lo as u128)
}

fn xoshiro256ss() -> u64 {
    let s0 = STATE[0].load(Ordering::Relaxed);
    let s1 = STATE[1].load(Ordering::Relaxed);
    let s2 = STATE[2].load(Ordering::Relaxed);
    let s3 = STATE[3].load(Ordering::Relaxed);

    let result = rotl(s1.wrapping_mul(5), 7).wrapping_mul(9);
    let t = s1 << 17;

    STATE[2].fetch_xor(s0, Ordering::Relaxed);
    STATE[3].fetch_xor(s1, Ordering::Relaxed);
    STATE[1].fetch_xor(s2, Ordering::Relaxed);
    STATE[0].fetch_xor(s3, Ordering::Relaxed);
    STATE[2].fetch_xor(t, Ordering::Relaxed);
    STATE[3]
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x| Some(rotl(x, 45)))
        .unwrap();

    result
}
