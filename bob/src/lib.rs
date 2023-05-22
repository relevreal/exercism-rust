pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.ends_with('?') => {
            if is_uppercase(m) {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        }
        m if is_uppercase(m) => "Whoa, chill out!",
        m if m.is_empty() || m.chars().all(|c| c.is_whitespace()) => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

fn is_uppercase(m: &str) -> bool {
    let mut iter = m.chars().filter(|c| c.is_ascii_alphabetic());
    match iter.clone().next() {
        Some(_) => iter.all(|c| c.is_uppercase()),
        _ => false,
    }
}
