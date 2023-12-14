use std::collections::HashMap;
use std::path::Path;
use itertools::Itertools;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<Vec<char>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let s = strs.split(' ').collect::<Vec<&str>>();
            let springs = s[0].chars().collect::<Vec<char>>();
            data.push(springs);
        });

    data
}

fn sort_line(data: Vec<char>, space_in_end: bool) -> Vec<char> {
    data.iter()
        .join("")
        .split("#")
        .map(|s| {
            if space_in_end {
                s.chars().sorted().rev().collect::<String>()
            } else {
                s.chars().sorted().collect::<String>()
            }
        })
        .join("#").chars().collect::<Vec<char>>()
}

fn roate_platform(data: &mut Vec<Vec<char>>, step2: bool) {
    // north
    for col_idx in 0..data[0].len() {
        let mut col_chars = vec![];
        for row_id in 0..data.len() {
            col_chars.push(data[row_id][col_idx]);
        }
        let sorted_col_chars = sort_line(col_chars, true);
        for row_id in 0..data.len() {
            data[row_id][col_idx] = sorted_col_chars[row_id];
        }
    }

    if step2 {
        // to west <-
        for row_idx in 0..data.len() {
            data[row_idx] = sort_line(data[row_idx].clone(), true);
        }

        // to south V
        for col_idx in 0..data[0].len() {
            let mut col_chars = vec![];
            for row_id in 0..data.len() {
                col_chars.push(data[row_id][col_idx]);
            }
            let sorted_col_chars = sort_line(col_chars, false);
            for row_id in 0..data.len() {
                data[row_id][col_idx] = sorted_col_chars[row_id];
            }
        }

        // to east ->
        for row_idx in 0..data.len() {
            data[row_idx] = sort_line(data[row_idx].clone(), false);
        }
    }
}

fn get_total_load(data: Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let data_len = data.len();
    for (idx, row) in data.iter().enumerate() {
        count += (row.iter().filter(|c| **c == 'O').count() * (data_len - idx)) as u64;
    }
    count
}

fn get_rotate_data(data: &mut Vec<Vec<char>>, times: usize) -> u64 {
    // fill the original one in found data
    let mut found_vec = vec![data.clone()];
    let mut found_map: HashMap<Vec<Vec<char>>, bool> = HashMap::from([
        (data.clone(), true)
    ]);

    let mut iter = 0;
    loop {
        iter += 1;
        roate_platform(data, true);
        if found_map.get(data).is_some() {
            break;
        }
        found_map.insert(data.clone(), true);
        found_vec.push(data.clone());
    }
    let start_idx = found_vec.iter().position(|line| line.eq(data)).unwrap();
    let final_idx = (times - start_idx) % (iter - start_idx) + start_idx;
    let result = get_total_load(found_vec[final_idx].clone());
    result
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn day14_1_test() {
        let mut data = format_data(Path::new("src/day14/day14_input_test.txt"));
        roate_platform(&mut data, false);
        let result = get_total_load(data);
        assert_eq!(result, 136);
    }

    #[test]
    fn day14_1_answer() {
        let mut data = format_data(Path::new("src/day14/day14_input.txt"));
        roate_platform(&mut data, false);
        let result = get_total_load(data);
        assert_eq!(result, 110677);
    }

    #[test]
    fn day14_2_test() {
        let mut data = format_data(Path::new("src/day14/day14_input_test.txt"));
        let result = get_rotate_data(&mut data, 1000000000);
        assert_eq!(result, 64);
    }

    #[test]
    fn day14_2_answer() {
        let mut data = format_data(Path::new("src/day14/day14_input.txt"));
        let result = get_rotate_data(&mut data, 1000000000);
        assert_eq!(result, 90551);
    }
}
