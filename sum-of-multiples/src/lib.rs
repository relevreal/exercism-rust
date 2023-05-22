use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&f| *f != 0)
        .map(|&f| {
            (f..)
                .step_by(f as usize)
                .take_while(|&n| n < limit)
                .collect::<HashSet<_>>()
        })
        .fold(HashSet::<u32>::new(), |res, new| {
            res.union(&new).copied().collect()
        })
        .drain()
        .sum()
}
