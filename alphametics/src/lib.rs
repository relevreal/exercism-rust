use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut all_chars = HashSet::<char>::new();
    let mut first_chars = HashSet::<char>::new();
    let mut addens: Vec<Vec<char>> = vec![];
    let mut adden: Vec<char> = vec![];
    let mut bytes_iter = input.bytes();
    while let Some(b) = bytes_iter.next() {
        match b {
            b' ' => {
                first_chars.insert(adden[0]);
                addens.push(adden);
                adden = vec![];
                bytes_iter.next();
                if let Some(b'=') = bytes_iter.next() {
                    bytes_iter.next();
                }
            }
            b => {
                adden.push(b as char);
                all_chars.insert(b as char);
            }
        }
    }
    first_chars.insert(adden[0]);
    let result = adden;

    let all_chars: Vec<char> = all_chars.drain().collect();
    let mut char_assignment = [10_u64; 26];
    let mut number_used = [false; 10];
    find(
        &addens,
        &result,
        &first_chars,
        &all_chars,
        &mut number_used,
        &mut char_assignment,
    )
}

fn find(
    addens: &[Vec<char>],
    expected_result: &[char],
    first_chars: &HashSet<char>,
    all_chars: &[char],
    number_used: &mut [bool],
    char_assignment: &mut [u64],
) -> Option<HashMap<char, u8>> {
    match all_chars.first() {
        Some(&c) => {
            for i in 0..10 {
                if number_used[i] || (i == 0 && first_chars.contains(&c)) {
                    continue;
                }
                number_used[i] = true;
                char_assignment[(c as u8 - b'A') as usize] = i as u64;
                let result = find(
                    addens,
                    expected_result,
                    first_chars,
                    &all_chars[1..],
                    number_used,
                    char_assignment,
                );
                if result.is_some() {
                    return result;
                }
                number_used[i] = false;
            }
            char_assignment[(c as u8 - b'A') as usize] = 10;
            None
        }
        None => match is_ok(addens, expected_result, char_assignment) {
            true => {
                let mut result = HashMap::<char, u8>::new();

                for (i, n) in char_assignment.iter().enumerate() {
                    if *n >= 10 {
                        continue;
                    }
                    let c = (b'A' + i as u8) as char;
                    result.insert(c, *n as u8);
                }
                Some(result)
            }
            false => None,
        },
    }
}

fn is_ok(addens: &[Vec<char>], expected_result: &[char], char_assignment: &[u64]) -> bool {
    let left_side: u64 = addens
        .iter()
        .map(|v| vec_to_number(v, char_assignment))
        .sum();
    let result = vec_to_number(expected_result, char_assignment);
    left_side == result
}

fn vec_to_number(v: &[char], char_assignment: &[u64]) -> u64 {
    let mut number = 0_u64;
    for (i, &c) in v.iter().rev().enumerate() {
        number += char_assignment[(c as u8 - b'A') as usize] * 10_u64.pow(i as u32);
    }
    number
}
