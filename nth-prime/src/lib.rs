pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        _ => {
            let mut candidate = 1;
            let mut seen_primes = 0;
            while seen_primes < n {
                candidate += 2;
                if is_prime(candidate) {
                    seen_primes += 1;
                }
            }
            candidate
        }
    }
}

fn is_prime(n: u32) -> bool {
    let n_sqrt = (n as f64).sqrt() as u32 + 1;
    let mut div = 2_u32;
    while div <= n_sqrt {
        if n % div == 0 {
            return false;
        }
        div += 1;
    }
    true
}

pub fn nth_alternative(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![2, 3];

    while primes.len() <= n as usize {
        let mut next_candidate = primes.last().unwrap_or(&0) + 2;
        while primes.iter().any(|prime| next_candidate % prime == 0) {
            next_candidate += 2;
        }
        primes.push(next_candidate);
    }
    primes[n as usize]
}
