const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return vec![];
    }

    let height = minefield.len() as i32;
    let width = minefield[0].len() as i32;
    minefield
        .iter()
        .enumerate()
        .map(|(row_i, &row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(col_i, &cell)| match cell {
                    c if c == b' ' => {
                        let count = DIRECTIONS
                            .iter()
                            .map(|&(row_dir, col_dir)| {
                                (row_i as i32 + row_dir, col_i as i32 + col_dir)
                            })
                            .filter(|&(x, y)| x >= 0 && y >= 0 && x < height && y < width)
                            .map(|(x, y)| minefield[x as usize].as_bytes()[y as usize] as char)
                            .filter(|&c| c == '*')
                            .count();
                        match count {
                            0 => ' ',
                            n => (n as u8 + b'0') as char,
                        }
                    }
                    c => c as char,
                })
                .collect()
        })
        .collect()
}
