pub fn square(s: u32) -> u64 {
    match s {
        s if s > 0 && s <= 64 => 2_u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}

pub fn total_more_efficient() -> u64 {
    let mut exp = 1_u64;
    let mut sum = 1_u64;
    for _ in 1..64 {
        exp *= 2;
        sum += exp;
    }
    sum
}

// All 64 bits set to one
pub fn total_cool_trick() -> u64 {
    u64::MAX
}
