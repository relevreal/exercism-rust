use std::cmp::Ordering;

const VERSES: [&str; 3] = [
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
    "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",
    "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n",
];

pub fn verse(n: u32) -> String {
    match n as usize {
        n if n <= 2 => VERSES[n].to_string(),
        n => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n - 1),
    }
}

fn map_verse(n: u32, last: u32) -> String {
    match n != last {
        true => verse(n) + "\n",
        false => verse(n),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let f = |n| map_verse(n, end);
    match start.cmp(&end) {
        Ordering::Less => (start..=end).map(f).collect(),
        Ordering::Greater => (end..=start).rev().map(f).collect(),
        Ordering::Equal => verse(start),
    }
}
