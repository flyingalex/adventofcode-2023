use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::vec;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<Vec<char>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            data.push(strs.chars().collect());
        });
    data
}

fn bread_first_search(mark: Vec<(usize, usize)> , map: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize), result: &mut HashSet<usize>) {
    if start == end {
        result.insert(mark.len());
        return;
    }

    for i in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let next = (start.0 as i32 + i.0, start.1 as i32 + i.1);
        if next.0 >= 0 && next.0 < map.len() as i32 && next.1 >= 0 && next.1 < map[0].len() as i32 {
            let next = (next.0 as usize, next.1 as usize);
            if !mark.contains(&next) && ['.', '^', 'v', '<', '>'].contains(&map[next.0][next.1]) {
                let mut mark = mark.clone();
                mark.push(next);
                bread_first_search(mark, map, (next.0, next.1), end, result);
            }
        }
    }
}

fn get_longest_hikes(map: Vec<Vec<char>>) -> usize {
    let mut result = HashSet::new();
    let start_y = map[0].iter().position(|c| *c == '.').unwrap();
    let end_y = map[map.len() - 1].iter().position(|c| *c == '.').unwrap();
    let start = (0, start_y);
    let end = (map.len() - 1, end_y);
    let mark: Vec<(usize, usize)> = vec![];
    bread_first_search(mark, &map, start, end, &mut result);
    println!("{:?}", result);
    result.iter().max().unwrap().to_owned()
}

#[cfg(test)]
mod day23_tests {
    use super::*;

    #[test]
    fn day23_1_test() {
        let data = format_data(Path::new("src/day23/day23_input_test.txt"));
        let result = get_longest_hikes(data);
        assert_eq!(result, 94);
    }

    #[test]
    fn day23_1_answer() {
        let data = format_data(Path::new("src/day23/day23_input.txt"));
        let result = get_longest_hikes(data);
        assert_eq!(result, 2042);
    }

    #[test]
    fn day23_2_test() {
        let data = format_data(Path::new("src/day23/day23_input_test.txt"));
        let result = get_longest_hikes(data);
        assert_eq!(result, 7);
    }

    #[test]
    fn day23_2_answer() {
        let data = format_data(Path::new("src/day23/day23_input.txt"));
        let result = get_longest_hikes(data);
        assert_eq!(result, 128163929109524);
    }
}

