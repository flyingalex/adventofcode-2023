use std::path::Path;
use itertools::Itertools;
use num::Integer;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<Vec<char>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let line_data = strs.chars().collect::<Vec<char>>();
            if line_data.iter().all(|&c| c == '.') {
                data.push(line_data.clone());
            }
            data.push(line_data);
        });
    // check if all spaces in one column is *
    let row_len = data.len();
    let col_len = data[0].len();
    let mut no_galxies_cols = vec![];
    for col_idx in 0..col_len {
        let mut has_galaxy = false;
        for rox_idx in 0..row_len {
            if data[rox_idx][col_idx] != '.' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            no_galxies_cols.insert(0, col_idx);
        }
    }

    for col_idx in no_galxies_cols {
        for row_idx in 0..row_len {
            data[row_idx].insert(col_idx, '.');
        }
    }
    data
}

fn get_next_points(current: &(usize, usize), max: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if current.0 > 0 {
        result.push((current.0 - 1, current.1));
    }
    if current.0 < max.0 {
        result.push((current.0 + 1, current.1));
    }
    if current.1 > 0 {
        result.push((current.0, current.1 - 1));
    }
    if current.1 < max.1 {
        result.push((current.0, current.1 + 1));
    }
    result
}

fn get_len_sum(data: &Vec<Vec<char>>) -> u64 {
    let mut glaxies = vec![];
    for (row_idx, row) in data.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '#' {
                glaxies.push((row_idx, col_idx));
            }
        }
    }
    let pairs = glaxies.iter().tuple_combinations().map(|(p1, p2)| (p1, p2)).collect::<Vec<(&(usize, usize), &(usize, usize))>>();
    pairs.iter().map(|(&p1, &p2)| {
        ((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as u64
    }).sum()
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn day11_1_test() {
        let data = format_data(Path::new("src/day11/day11_input_test.txt"));
        let result = get_len_sum(&data);
        assert_eq!(result, 374);
    }

    #[test]
    fn day11_1_answer() {
        let data = format_data(Path::new("src/day11/day11_input.txt"));
        let result = get_len_sum(&data);
        assert_eq!(result, 6831);
    }

    #[test]
    fn day11_2_test() {
        let data = format_data(Path::new("src/day11/day11_input_test2.txt"));
        let result = get_len_sum(&data);
        assert_eq!(result, 10);
    }

    #[test]
    fn day11_2_answer() {
        let data = format_data(Path::new("src/day11/day11_input.txt"));
        let result = get_len_sum(&data);
        assert_eq!(result, 305);
    }
}
