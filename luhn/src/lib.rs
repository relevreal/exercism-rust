/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }
    let mut sum = 0;
    for (i, c) in code
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .enumerate()
    {
        if c.is_ascii_digit() {
            let mut d = c.to_digit(10).unwrap();
            if i % 2 == 1 {
                d *= 2;
            }
            if d > 9 {
                d -= 9;
            }
            sum += d;
        } else {
            return false;
        }
    }
    sum % 10 == 0
}
