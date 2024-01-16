use crate::Solution;

struct Day1;
struct Part1;
struct Part2;

trait Part {
    fn find_digit(&self, idx: usize, chars: &[char]) -> Option<u32>;
}
impl Part for Part2 {
    fn find_digit(&self, idx: usize, chars: &[char]) -> Option<u32> {
        if chars[idx].is_ascii_digit() {
            return chars[idx].to_digit(10);
        }

        let digits: Vec<&str> = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let len = chars.len();
        // check numbr word with 3,4,5 characters
        for r in [2, 3, 4] {
            if (idx + r) < len {
                let s = chars[idx..=idx + r].iter().collect::<String>();
                if let Some(found) = digits.iter().position(|&d| d == s) {
                    return Some(found as u32 + 1);
                }
            }
        }

        None
    }
}

impl Part for Part1 {
    fn find_digit(&self, idx: usize, chars: &[char]) -> Option<u32> {
        chars[idx].to_digit(10)
    }
}
impl Day1 {
    pub fn calibration_value<P: Part>(&self, part: P, input: &str) -> u32 {
        input
            .lines()
            .map(|l| {
                let mut first = None;
                let mut last = None;
                let mut first_idx = 0;
                let chars = l.chars().collect::<Vec<char>>();
                let mut last_idx = chars.len() - 1;

                loop {
                    if first.is_none() {
                        first = part.find_digit(first_idx, &chars);
                    }

                    if last.is_none() {
                        last = part.find_digit(last_idx, &chars);
                    }

                    if first.is_some() && last.is_some() {
                        break;
                    }
                    first_idx += 1;
                    last_idx -= 1;
                }
                (first.unwrap() * 10) + last.unwrap()
            })
            .sum()
    }
}
impl Solution for Day1 {
    fn part1_test(&self) -> u32 {
        self.calibration_value(Part1, include_str!("day1_input_test1.txt"))
    }
    fn part1(&self) -> u32 {
        self.calibration_value(Part1, include_str!("day1_input.txt"))
    }

    fn part2_test(&self) -> u32 {
        self.calibration_value(Part2, include_str!("day1_input_test2.txt"))
    }

    fn part2(&self) -> u32 {
        self.calibration_value(Part2, include_str!("day1_input.txt"))
    }
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn day1_1_test() {
        assert_eq!(Day1.part1_test(), 142);
    }

    #[test]
    fn day1_1_answer() {
        assert_eq!(Day1.part1(), 55621);
    }

    #[test]
    fn day1_2_test() {
        assert_eq!(Day1.part2_test(), 281);
    }

    #[test]
    fn day1_2_answer() {
        assert_eq!(Day1.part2(), 53592);
    }
}
