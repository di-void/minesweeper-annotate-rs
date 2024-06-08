const MINE: u8 = b'*';
const SPACE: u8 = b' ';

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let fh = minefield.len();
    let fw = if fh > 0 { minefield[0].len() } else { 0 };
    let mut annotated_minefield: Vec<Vec<u8>> = vec![vec![b' '; fw]; fh];

    if fh == 0 {
        return vec![];
    }

    if fw == 0 {
        return vec!["".to_string()];
    }

    for (row_index, row) in minefield.iter().enumerate() {
        for (col_index, &cell) in row.as_bytes().iter().enumerate() {
            if cell == MINE {
                annotated_minefield[row_index][col_index] = MINE;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let ni = row_index as isize + dx;
                        let nj = col_index as isize + dy;
                        if in_bounds(ni, nj, fh, fw)
                            && annotated_minefield[ni as usize][nj as usize] != MINE
                        {
                            annotated_minefield[ni as usize][nj as usize] =
                                match annotated_minefield[ni as usize][nj as usize] {
                                    SPACE => b'1',
                                    n => n + 1,
                                };
                        }
                    }
                }
            }
        }
    }

    annotated_minefield
        .into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}

fn in_bounds(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    x >= 0 && x < rows as isize && y >= 0 && y < cols as isize
}
