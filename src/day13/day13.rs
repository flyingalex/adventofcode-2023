use std::path::Path;
use itertools::Itertools;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<Vec<Vec<char>>> {
    let mut data: Vec<Vec<Vec<char>>> = vec![];
    let mut temp = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            if strs.is_empty() && !temp.is_empty() {
                data.push(temp.clone());
                temp = vec![];
            } else {
                let s = strs.chars().collect::<Vec<char>>();
                temp.push(s);
            }
        });
    // check if all spaces in one column is *
    data.push(temp);
    data
}

fn get_notes_sum(data: Vec<Vec<Vec<char>>>) -> usize {
    let mut count = 0;
    for mirror_map in data {
        // row/col compare
        let mut row_mirror_count = 0;
        let mut col_mirror_count = 0;
        for (row_i, row) in mirror_map.windows(2).enumerate() {
            if row[0].eq(&row[1]) {
                let mut found = true;
                for top_idx in (0..=row_i).rev() {
                    let bottom_idx = 2 * row_i - top_idx + 1;
                    if bottom_idx < mirror_map.len() && !mirror_map[top_idx].eq(&mirror_map[bottom_idx]) {
                        found = false;
                    }
                }
                if found {
                    row_mirror_count = row_i + 1;
                    break;
                }
            }
        }

        for col_i in 0..(mirror_map[0].len() - 1) {
            if mirror_map.iter().all(|row| {
                row[col_i].eq(&row[col_i + 1])
            }) {
                let mut found = true;
                for left_idx in (0..=col_i).rev() {
                    let right_idx = 2 * col_i - left_idx + 1;
                    if right_idx < mirror_map[0].len() && !mirror_map.iter().all(|row| {
                        row[left_idx].eq(&row[right_idx])
                    }) {
                        found = false;
                    }
                }
                if found {
                    col_mirror_count = col_i + 1;
                    break;
                }
            }
        }
        if row_mirror_count > col_mirror_count {
            count += row_mirror_count * 100;
        } else {
            count += col_mirror_count;
        }
    }
    count
}

fn get_notes_sum2(data: Vec<Vec<Vec<char>>>) -> usize {
    let mut count = 0;
    for mirror_map in data {
        // row/col compare
        let mut row_mirror_count = 0;
        let mut col_mirror_count = 0;
        for row_idxs in (0..mirror_map.len()).combinations(2) {
            let rows = vec![&mirror_map[row_idxs[0]], &mirror_map[row_idxs[1]]];
            let mut diff = 0;
            for pair in rows[0].iter().zip(rows[1].iter()) {
                if pair.0.ne(pair.1) {
                    diff += 1;
                }
            }
            if diff == 1 {
                let row0_idx = row_idxs[0];
                let row1_idx = row_idxs[1];
                let new_row_mirror_count_idx = (row0_idx + row1_idx).div_ceil(2) - 1;
                let mut is_new_valid = true;
                let mut diff_count = 0;
                for top_idx in (0..=new_row_mirror_count_idx).rev() {
                    let bottom_idx = 2 * new_row_mirror_count_idx - top_idx + 1;
                    if bottom_idx < mirror_map.len() {
                        for pair in mirror_map[top_idx].iter().zip(mirror_map[bottom_idx].iter()) {
                            if pair.0.ne(pair.1) {
                                diff_count += 1;
                            }
                        }
                    }
                }
                if diff_count != 1 {
                    is_new_valid = false;
                }
                if is_new_valid {
                    row_mirror_count = new_row_mirror_count_idx + 1;
                    break;
                }
            }
        }

        for col_i in (0..mirror_map[0].len()).into_iter().combinations(2) {
            let mut diff = 0;
            for row_idx in 0..mirror_map.len() {
                if mirror_map[row_idx][col_i[0]].ne(&mirror_map[row_idx][col_i[1]]) {
                    diff += 1;
                }
            }
            if diff == 1 {
                let new_col_mirror_count_idx = (col_i[0] + col_i[1]).div_ceil(2) - 1;
                // check if new one is valid
                let mut is_new_valid = true;
                let mut diff_count = 0;
                for left_idx in (0..=new_col_mirror_count_idx).rev() {
                    let right_idx = 2 * new_col_mirror_count_idx - left_idx + 1;
                    if right_idx < mirror_map[0].len() {
                        mirror_map.iter().for_each(|row| {
                            if row[left_idx].ne(&row[right_idx]) {
                                diff_count += 1;
                            }
                        });
                    }
                }
                if diff_count != 1 {
                    is_new_valid = false;
                }

                if is_new_valid {
                    col_mirror_count = new_col_mirror_count_idx + 1;
                    break;
                }
            }
        }

        if row_mirror_count > col_mirror_count {
            count += row_mirror_count * 100;
        } else {
            count += col_mirror_count;
        }
    }
    count
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn day13_1_test() {
        let data = format_data(Path::new("src/day13/day13_input_test.txt"));
        let result = get_notes_sum(data);
        assert_eq!(result, 405);
    }

    #[test]
    fn day13_1_answer() {
        let data = format_data(Path::new("src/day13/day13_input.txt"));
        let result = get_notes_sum(data);
        assert_eq!(result, 27742);
    }

    #[test]
    fn day13_2_test() {
        let data = format_data(Path::new("src/day13/day13_input_test.txt"));
        let result = get_notes_sum2(data);
        // col 3, 1, 5
        assert_eq!(result, 400);
    }

    #[test]
    fn day13_3_test() {
        let data = format_data(Path::new("src/day13/day13_input_test2.txt"));
        let result = get_notes_sum2(data);
        // col 3, 1, 5
        assert_eq!(result, 10);
    }

    #[test]
    fn day13_4_test() {
        let data = format_data(Path::new("src/day13/day13_input_test3.txt"));
        let result = get_notes_sum2(data);
        // col 3, 1, 5
        assert_eq!(result, 1100);
    }

    #[test]
    fn day13_5_test() {
        let data = format_data(Path::new("src/day13/day13_input_test4.txt"));
        let result = get_notes_sum2(data);
        // col 3, 1, 5
        assert_eq!(result, 1600);
    }

    #[test]
    fn day13_2_answer() {
        let data = format_data(Path::new("src/day13/day13_input.txt"));
        let result = get_notes_sum2(data);
        // too low
        assert_eq!(result, 32728);
    }
}
