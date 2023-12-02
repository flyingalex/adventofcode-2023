use std::path::Path;
use crate::utils::read_lines;

fn is_digit_word(idx: usize, chars: &Vec<char>) -> Option<u32> {
    let digits: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let len = chars.len();
    // check numbr word with 3,4,5 characters
    for r in [2, 3, 4] {
        if (idx + r) < len {
            if let Some(found) = digits.iter().position(|&d| d == chars[idx..=idx + r].iter().collect::<String>()) {
                return Some(found as u32 + 1);
            }
        }
    }

    None
}

fn find_digit(idx: usize, chars: &Vec<char>, check_digit_word: bool) -> u32 {
    let mut found = 0;
    let c = chars[idx];
    // check digit number
    if c.is_ascii_digit() {
        found = c.to_digit(10).unwrap();
        // check digit word
    } else if check_digit_word && let Some(num) = is_digit_word(idx, chars) {
        found = num;
    }
    found
}

pub fn calibration_value(file: &Path, check_digit_word: bool) -> u32 {
    read_lines(file)
        .unwrap()
        .map(|l| {
            let mut first = 0;
            let mut last = 0;
            let mut first_idx = 0;
            let chars = l.unwrap().chars().collect::<Vec<char>>();
            let mut last_idx = chars.len() - 1;

            loop {
                if first == 0 {
                    first = find_digit(first_idx, &chars, check_digit_word);
                }

                if last == 0 {
                    last = find_digit(last_idx, &chars, check_digit_word);
                }

                if first != 0 && last != 0 {
                    break
                }
                first_idx += 1;
                last_idx -= 1;
            }
            (first * 10) + last
        })
        .sum()
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn day1_1_test() {
        let result = calibration_value(Path::new("src/day1/day1_input_test1.txt"), false);
        assert_eq!(result, 142);
    }

    #[test]
    fn day1_1_answer() {
        let result = calibration_value(Path::new("src/day1/day1_input.txt"), false);
        assert_eq!(result, 55621);
    }

    #[test]
    fn day1_2_test() {
        let result = calibration_value(Path::new("src/day1/day1_input_test2.txt"), true);
        assert_eq!(result, 281);
    }

    #[test]
    fn day1_2_answer() {
        let result = calibration_value(Path::new("src/day1/day1_input.txt"), true);
        assert_eq!(result, 53592);
    }
}
