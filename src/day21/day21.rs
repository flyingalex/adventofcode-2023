use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::vec;
use crate::utils::read_lines;

fn format_data(file: &Path) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut data = vec![];
    let mut start = (0, 0);
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let chars = strs.chars().collect::<Vec<char>>();
            if let Some(idx) = chars.iter().position(|&c| c == 'S') {
                start = (data.len() as i32, idx as i32);
            }
            data.push(strs.chars().collect::<Vec<char>>());
        });
    (data, start)
}

fn get_garden_plots(data: Vec<Vec<char>>, start: (i32, i32), step: i32) -> u64 {
    let mut steps = HashSet::new();
    steps.insert(start);
    let mut next_steps = HashSet::new();
    let mut plots = HashSet::new();
    for i in 0..step {
        for (x, y) in steps {
            for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < data.len() as i32 &&
                    ny >= 0 && ny < data[0].len() as i32 &&
                    (data[nx as usize][ny as usize] == '.' || data[nx as usize][ny as usize] == 'S') {
                    next_steps.insert((nx, ny));
                    if i == step - 1 {
                        plots.insert((nx, ny));
                    }
                }
            }
        }
        steps = next_steps;
        next_steps = HashSet::new();
    }
    plots.len() as u64
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn day21_1_test() {
        let (data, start) = format_data(Path::new("src/day21/day21_input_test.txt"));
        let result = get_garden_plots(data, start, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn day21_1_answer() {
        let (data, start) = format_data(Path::new("src/day21/day21_input.txt"));
        let result = get_garden_plots(data, start, 64);
        assert_eq!(result, 3841);
    }

    #[test]
    fn day21_2_test() {
        let (data, start) = format_data(Path::new("src/day21/day21_input_test.txt"));
        let result = get_garden_plots(data, start, 12);
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn day21_2_answer() {
        let (data, start) = format_data(Path::new("src/day21/day21_input.txt"));
        let result = get_garden_plots(data, start, 12);
        assert_eq!(result, 128163929109524);
    }
}

