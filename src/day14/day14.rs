use std::path::Path;
use crate::utils::read_lines;

fn format_data(file: &Path, step2: bool) -> Vec<Vec<char>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let s = strs.split(' ').collect::<Vec<&str>>();
            let springs = s[0].chars().collect::<Vec<char>>();
            data.push(springs);
        });
    // move rock to north
    for col_idx in 0..data[0].len() {
        let last_row_idx = data.len() - 1;
        let mut start_row_idx = 0;
        loop {
            // move rock to here if its empty
            if data[start_row_idx][col_idx] == '.' {
                for row_idx in start_row_idx..=last_row_idx {
                    if data[row_idx][col_idx] == 'O' {
                        data[row_idx][col_idx] = '.';
                        data[start_row_idx][col_idx] = 'O';
                        break;
                    }
                    if data[row_idx][col_idx] == '#' {
                        start_row_idx = row_idx;
                        break;
                    }
                }
            }
            start_row_idx += 1;

            if start_row_idx > last_row_idx {
                break;
            }
        }
    }

    data
}

fn get_total_load(data: Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let data_len = data.len();
    for (idx, row) in data.iter().enumerate() {
        println!("row {:?}", row);
        count += (row.iter().filter(|c| **c == 'O').count() * (data_len - idx)) as u64;
    }
    count
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    #[test]
    fn day14_1_test() {
        let data = format_data(Path::new("src/day14/day14_input_test.txt"), false);
        let result = get_total_load(data);
        assert_eq!(result, 136);
    }

    #[test]
    fn day14_1_answer() {
        let data = format_data(Path::new("src/day14/day14_input.txt"), false);
        let result = get_total_load(data);
        assert_eq!(result, 110677);
    }

    #[test]
    fn day14_2_test() {
        let data = format_data(Path::new("src/day14/day14_input_test.txt"), true);
        let result = get_total_load(data);
        assert_eq!(result, 525152);
    }

    #[test]
    fn day14_2_answer() {
        let data = format_data(Path::new("src/day14/day14_input.txt"), true);
        let result = get_total_load(data);
        assert_eq!(result, 50338344809230);
    }
}
