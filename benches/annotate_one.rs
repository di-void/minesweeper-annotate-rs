// byte values
const MINE: u8 = b'*'; // '*'
const SPACE: u8 = b' '; // ' '

#[derive(Debug)]
enum POSITION {
    TrTc(u8, Check),
    TrIc(u8, u8, Check),
    IrTc(u8, Check, Check),
    IrIc(u8, u8, Check, Check),
}

#[derive(Debug)]
enum Check {
    Up(u8, u8, Option<u8>),
    Down(u8, u8, u8, Option<u8>),
}

use self::Check::{Down, Up};

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let fh = minefield.len();
    let fw = if fh > 0 { minefield[0].len() } else { 0 };
    let mut annotated_minefield: Vec<Vec<u8>> = Vec::new();

    if fh == 0 {
        ()
    } else if fw == 0 {
        return vec!["".to_string()];
    } else {
        let mut incomplete_checks = vec![];

        for (row_index, row) in minefield.iter().enumerate() {
            let owned_row = Vec::from(row.as_bytes());
            annotated_minefield.push(owned_row.clone());

            for (col_index, &cell) in owned_row.iter().enumerate() {
                if cell == MINE {
                    match is_terminus_or_inter(row_index as u8, col_index as u8, fw as u8, fh as u8)
                    {
                        POSITION::TrTc(check_index, check) => {
                            update_adjacent(
                                check_index,
                                check_index,
                                &mut annotated_minefield,
                                row_index,
                            );

                            match check {
                                Up(lower_bound, mid, max) => {
                                    let upper_bound = max.unwrap_or(mid);
                                    update_adjacent(
                                        lower_bound,
                                        upper_bound,
                                        &mut annotated_minefield,
                                        row_index - 1,
                                    );
                                }
                                _ => incomplete_checks.push(check),
                            }
                        }
                        POSITION::TrIc(check_index_one, check_index_two, check) => {
                            update_adjacent(
                                check_index_one,
                                check_index_two,
                                &mut annotated_minefield,
                                row_index,
                            );

                            match check {
                                Up(min, mid, max) => {
                                    let max = max.unwrap_or(mid);
                                    update_adjacent(
                                        min,
                                        max,
                                        &mut annotated_minefield,
                                        row_index - 1,
                                    );
                                }
                                _ => incomplete_checks.push(check),
                            }
                        }
                        POSITION::IrTc(check_index, check_up, check_down) => {
                            update_adjacent(
                                check_index,
                                check_index,
                                &mut annotated_minefield,
                                row_index,
                            );
                            incomplete_checks.push(check_down);

                            if let Up(min, mid, max) = check_up {
                                let max = max.unwrap_or(mid);
                                update_adjacent(min, max, &mut annotated_minefield, row_index - 1);
                            }
                        }
                        POSITION::IrIc(check_left, check_right, check_up, check_down) => {
                            update_adjacent(
                                check_left,
                                check_right,
                                &mut annotated_minefield,
                                row_index,
                            );
                            incomplete_checks.push(check_down);

                            if let Up(min, mid, max) = check_up {
                                let max = max.unwrap_or(mid);
                                update_adjacent(min, max, &mut annotated_minefield, row_index - 1);
                            }
                        }
                    }
                }

                if incomplete_checks.len() > 0 && row_index > 0 {
                    if let Down(row, lower_bound, mid, max) = incomplete_checks[0] {
                        if col_index as u8 > mid {
                            continue;
                        } else if row_index as u8 == row {
                            let upper_bound = max.unwrap_or(mid);

                            update_adjacent(
                                lower_bound,
                                upper_bound,
                                &mut annotated_minefield,
                                row_index,
                            );
                            incomplete_checks.remove(0);
                        }
                    }
                }
            }
        }
    }

    // annotated_minefield
    annotated_minefield
        .iter()
        .map(|byte_arr| String::from_utf8(byte_arr.clone()).unwrap())
        .collect()
}

fn update_adjacent(
    lower_bound: u8,
    upper_bound: u8,
    annotated_minefield: &mut Vec<Vec<u8>>,
    row_index: usize,
) {
    let one_byte = b'1';

    for v in lower_bound..=upper_bound {
        let check_row = &mut annotated_minefield[row_index];
        let cell = &mut check_row[v as usize];

        if *cell == SPACE {
            *cell = one_byte;
        } else if is_valid_count_byte(*cell) {
            *cell += 1;
        }
    }
}

fn is_valid_count_byte(byte: u8) -> bool {
    match byte {
        49..=55 => true,
        _ => false,
    }
}

// adjacent checkpoints

// t: terminus
// i: intermmediate
// r: row
// c: col

// hr => 2; (left and right)
// tr x tc => 3 = t(r x c) => 3 (space search)
// tr x ic => 4 (mine search)
// ir x tc => 5
// ir x ic => 8 = i(r x c) => 8

// fw: field width; fh: field height
fn is_terminus_or_inter(row_index: u8, col_index: u8, fw: u8, fh: u8) -> POSITION {
    let tr = row_index == 0 || row_index == fh - 1;
    let ir = row_index > 0 && row_index < fh - 1;
    let tc = col_index == 0 || col_index == fw - 1;
    let ic = col_index > 0 && col_index < fw - 1;
    let safe_index = 1 % fw;
    let incr_col_index = col_index + 1;
    let decr_col_index = col_index as i8 - 1;
    let incr_row_index = row_index + 1;

    if tr && tc {
        // there are 4 possible mine positions for trtc
        // a: (0,0), b: (0,fw - 1), c: (fh - 1, 0), d: (fh - 1, fw - 1)

        let pos_res = row_index + col_index;
        let neg_res = row_index as i8 - col_index as i8;

        // checkpoints
        if pos_res == 0 {
            return POSITION::TrTc(safe_index, Down(incr_row_index, 0, safe_index, None));
        }

        if neg_res < 0 {
            return POSITION::TrTc(
                pos_res - 1,
                Down(incr_row_index, pos_res - 1, pos_res, None),
            );
        }

        if neg_res > 0 && col_index < fw - 1 {
            return POSITION::TrTc(safe_index, Up(0, safe_index, None));
        }

        return POSITION::TrTc(
            col_index - safe_index,
            Up(col_index - safe_index, col_index, None),
        );
    } else if tr && ic {
        if row_index < col_index {
            return POSITION::TrIc(
                decr_col_index as u8,
                incr_col_index,
                Down(
                    incr_row_index,
                    decr_col_index as u8,
                    col_index,
                    Some(incr_col_index),
                ),
            );
        } else {
            return POSITION::TrIc(
                decr_col_index as u8,
                incr_col_index,
                Up(decr_col_index as u8, col_index, Some(incr_col_index)),
            );
        }
    } else if ir && tc {
        if decr_col_index < 0 {
            return POSITION::IrTc(
                safe_index,
                Up(col_index, safe_index, None),
                Down(incr_row_index, col_index, safe_index, None),
            );
        }

        return POSITION::IrTc(
            decr_col_index as u8,
            Up(decr_col_index as u8, col_index, None),
            Down(incr_row_index, decr_col_index as u8, col_index, None),
        );
    } else {
        // field will have at least 3 columns and 3 rows
        return POSITION::IrIc(
            decr_col_index as u8,
            incr_col_index,
            Up(decr_col_index as u8 as u8, col_index, Some(incr_col_index)),
            Down(
                incr_row_index,
                decr_col_index as u8,
                col_index,
                Some(incr_col_index),
            ),
        );
    }
}
