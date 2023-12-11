use std::path::Path;
use itertools::Itertools;
use num::Integer;
use crate::utils::read_lines;

fn format_data(file: &Path) -> (Vec<Vec<char>>, (Vec<usize>, Vec<usize>)) {
    let mut data = vec![];
    let mut empty_row_idxs = vec![];
    let mut empty_col_idxs = vec![];
    let mut row_idx = -1;
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            row_idx += 1;
            let strs = l.unwrap();
            let line_data = strs.chars().collect::<Vec<char>>();
            if line_data.iter().all(|&c| c == '.') {
                empty_row_idxs.push(row_idx  as usize);
            }
            data.push(line_data);
        });
    // check if all spaces in one column is *
    let row_len = data.len();
    let col_len = data[0].len();
    for col_idx in 0..col_len {
        let mut has_galaxy = false;
        for rox_idx in 0..row_len {
            if data[rox_idx][col_idx] != '.' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            empty_col_idxs.push(col_idx);
        }
    }

    (data, (empty_row_idxs, empty_col_idxs))
}

fn get_len_sum(data: &Vec<Vec<char>>, empty_row_cols: &(Vec<usize>, Vec<usize>), times: u64) -> u64 {
    let mut glaxies = vec![];
    for (row_idx, row) in data.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '#' {
                glaxies.push((row_idx, col_idx));
            }
        }
    }
    let (empty_row_idxs, empty_col_idxs) = empty_row_cols;
    let pairs = glaxies.iter().tuple_combinations().map(|(p1, p2)| (p1, p2)).collect::<Vec<(&(usize, usize), &(usize, usize))>>();
    pairs.iter().map(|(&p1, &p2)| {
        let mut path = ((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as u64;
        let row_added = empty_row_idxs.iter().filter(|&idx| {
           p1.0.min(p2.0) < *idx && p1.0.max(p2.0) > *idx
        }).count();
        let col_added = empty_col_idxs.iter().filter(|&idx| {
            p1.1.min(p2.1) < *idx && p1.1.max(p2.1) > *idx
        }).count();

        path + (row_added as u64 * times + col_added as u64 * times)
    }).sum()
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn day11_1_test() {
        let (data, empty_row_cols) = format_data(Path::new("src/day11/day11_input_test.txt"));
        let result = get_len_sum(&data, &empty_row_cols, 1);
        assert_eq!(result, 374);
    }

    #[test]
    fn day11_1_answer() {
        let (data, empty_row_cols) = format_data(Path::new("src/day11/day11_input.txt"));
        let result = get_len_sum(&data, &empty_row_cols, 1);
        assert_eq!(result, 9648398);
    }

    #[test]
    fn day11_2_test() {
        let (data, empty_row_cols) = format_data(Path::new("src/day11/day11_input_test.txt"));
        let result = get_len_sum(&data, &empty_row_cols, 9);
        assert_eq!(result, 1030);
    }

    #[test]
    fn day11_2_answer() {
        let (data, empty_row_cols) = format_data(Path::new("src/day11/day11_input.txt"));
        let result = get_len_sum(&data, &empty_row_cols, 999999);
        assert_eq!(result, 305);
    }
}
