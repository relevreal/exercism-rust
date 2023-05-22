use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let input = input.join("").to_lowercase();
    let mut chars_iter = input.chars();
    let mut chunk_size = input.len() / worker_count;

    let handles: Vec<_> = (0..worker_count)
        .map(|i| {
            if i + 1 == worker_count {
                chunk_size = input.len() - (i * chunk_size);
            }
            let chunk: String = chars_iter.by_ref().take(chunk_size).collect();
            thread::spawn(move || {
                let mut local_counter = HashMap::new();

                for c in chunk.chars().filter(|c| c.is_alphabetic()) {
                    *local_counter.entry(c).or_insert(0) += 1;
                }

                local_counter
            })
        })
        .collect();

    let mut counter = HashMap::new();
    for handle in handles {
        let handle_counter = handle.join().unwrap();
        for (c, count) in handle_counter {
            *counter.entry(c).or_insert(0) += count;
        }
    }

    counter
}
