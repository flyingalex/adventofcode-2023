use std::path::Path;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<Vec<i64>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let line_data = strs.split(" ").filter(|s| !s.is_empty()).map(|s| {
                s.parse::<i64>().unwrap()
            }).collect();
            data.push(line_data);
        });
    data
}

fn get_extrapolated_values(histories: Vec<Vec<i64>>) -> i64 {
    let mut total = 0;
    for history in histories {
        let mut last_nums = vec![*history.last().unwrap()];
        let mut new_history = history.clone();
        loop {
            let mut new_created_history = vec![];
            for window in new_history.windows(2).into_iter() {
                new_created_history.push(window[1] - window[0]);
            }
            if new_history.iter().all(|&h| h == 0) {
                break;
            }
            last_nums.push(*new_created_history.last().unwrap());
            new_history = new_created_history;
        }
        // rule: plus all last value
        total += &last_nums.iter().sum();
    }
    total
}

fn get_extrapolated_values2(histories: Vec<Vec<i64>>) -> i64 {
    let mut total = 0;
    for history in histories {
        let mut first_nums = vec![*history.first().unwrap()];
        let mut new_history = history.clone();
        loop {
            let mut new_created_history = vec![];
            for window in new_history.windows(2).into_iter() {
                new_created_history.push(window[1] - window[0]);
            }
            if new_history.iter().all(|&h| h == 0) {
                break;
            }
            // inject to first, reverse it
            first_nums.insert(0, *new_created_history.first().unwrap());
            new_history = new_created_history;
        }
        // rule: (this is different) we need to subtract the last result
        total += first_nums.into_iter().reduce(|acc, e| e - acc).unwrap();
    }
    total
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn day9_1_test() {
        let histories = format_data(Path::new("src/day9/day9_input_test.txt"));
        let result = get_extrapolated_values(histories);
        assert_eq!(result, 114);
    }

    #[test]
    fn day9_1_answer() {
        let histories = format_data(Path::new("src/day9/day9_input.txt"));
        let result = get_extrapolated_values(histories);
        assert_eq!(result, 1641934234);
    }

    #[test]
    fn day9_2_test() {
        let histories = format_data(Path::new("src/day9/day9_input_test.txt"));
        let result = get_extrapolated_values2(histories);
        assert_eq!(result, 2);
    }

    #[test]
    fn day9_2_answer() {
        let histories = format_data(Path::new("src/day9/day9_input.txt"));
        let result = get_extrapolated_values2(histories);
        assert_eq!(result, -14929);
    }
}
