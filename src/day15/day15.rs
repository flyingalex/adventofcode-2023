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
            let s = strs.split(',').collect::<Vec<&str>>();
            let springs = s
                .iter()
                .map(|&s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            data = springs;
        });
    data
}

fn get_result_of_sequence(data: Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    for datum in data {
        let mut start = 0;
        datum.iter().for_each(|&c| {
            start = ((start + c as u32) * 17) % 256;
        });
        result += start as u64;
    }
    result
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn day15_1_test() {
        let data = format_data(Path::new("src/day15/day15_input_test.txt"));
        let result = get_result_of_sequence(data);
        assert_eq!(result, 136);
    }

    #[test]
    fn day15_1_answer() {
        let data = format_data(Path::new("src/day15/day15_input.txt"));
        let result = get_result_of_sequence(data);
        assert_eq!(result, 514639);
    }

    #[test]
    fn day15_2_test() {
        let data = format_data(Path::new("src/day15/day15_input_test.txt"));
        let result = get_result_of_sequence(data);
        assert_eq!(result, 64);
    }

    #[test]
    fn day15_2_answer() {
        let data = format_data(Path::new("src/day15/day15_input.txt"));
        let result = get_result_of_sequence(data);
        assert_eq!(result, 90551);
    }
}
