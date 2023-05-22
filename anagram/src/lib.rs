use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);

    let mut anagrams = HashSet::new();
    for &w in possible_anagrams {
        let candidate_lower = w.to_lowercase();
        if candidate_lower != word_lower {
            let candidate_sorted = get_sorted(&candidate_lower);
            if candidate_sorted == word_sorted {
                anagrams.insert(w);
            }
        }
    }
    anagrams
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}
