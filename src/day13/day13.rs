use std::collections::HashMap;
use std::path::Path;
use crate::utils::read_lines;

fn format_data(file: &Path, step2: bool) -> Vec<Vec<Vec<char>>> {
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
        // row compare
        let mut mirror_count = 0;
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
                    mirror_count = (row_i + 1) * 100;
                    break;
                }
            }
        }

        // column compare
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
                    mirror_count = col_i + 1;
                    break;
                }
            }
        }

        count += mirror_count;
    }
    count
}

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn day13_1_test() {
        let data = format_data(Path::new("src/day13/day13_input_test.txt"), false);
        let result = get_notes_sum(data);
        assert_eq!(result, 405);
    }

    #[test]
    fn day13_1_answer() {
        let data = format_data(Path::new("src/day13/day13_input.txt"), false);
        let result = get_notes_sum(data);
        assert_eq!(result, 27742);
    }

    #[test]
    fn day13_2_test() {
        let data = format_data(Path::new("src/day13/day13_input_test.txt"), true);
        let result = get_notes_sum(data);
        assert_eq!(result, 525152);
    }

    #[test]
    fn day13_2_answer() {
        let data = format_data(Path::new("src/day13/day13_input.txt"), true);
        let result = get_notes_sum(data);
        assert_eq!(result, 50338344809230);
    }
}
