pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = vec![];
    while n > 1 {
        let mut candidate = 2_u64;
        while n % candidate != 0 {
            if candidate == 2 {
                candidate = 3;
            } else {
                candidate += 2;
            }
        }
        prime_factors.push(candidate);
        n /= candidate;
    }
    prime_factors
}
