pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    for w in phrase.split(&[' ', '-', '_']).filter(|&w| !w.is_empty()) {
        if w.to_uppercase() == w {
            result.push(w.chars().next().unwrap());
        } else {
            let s: String = w
                .char_indices()
                .filter(|(i, c)| *i == 0 || c.is_uppercase())
                .map(|(_, c)| c.to_uppercase().collect::<String>())
                .collect();
            result.push_str(&s);
        }
    }
    result
}

pub fn abbreviate_much_better(phrase: &str) -> String {
    phrase
        .split(&[' ', '-', '_'])
        .flat_map(|w| {
            w.chars()
                .take(1)
                .map(|c| c.to_ascii_uppercase())
                .chain(w.chars().skip_while(|c| c.is_ascii_uppercase()))
                .filter(|c| c.is_ascii_uppercase())
        })
        .collect()
}
