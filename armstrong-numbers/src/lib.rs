pub fn is_armstrong_number(num: u32) -> bool {
    let mut n = num;
    let mut digits: Vec<u32> = vec![];
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let pow = digits.len() as u32;
    let mut sum = 0_u32;
    for d in digits.into_iter() {
        match sum.checked_add(d.pow(pow)) {
            Some(s) => sum = s,
            None => return false,
        }
    }
    sum == num
}
