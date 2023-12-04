use std::path::Path;
use crate::utils::read_lines;

fn total_points(file: &Path) -> u32 {
    read_lines(file)
        .unwrap()
        .map(|l| {
            let strs = l.unwrap();
            let line_numbers = strs.split(':').collect::<Vec<&str>>();
            let mut winning_numbers: Vec<&str> = vec![];
            let mut is_winding_number = true;
            let mut num = 0;
            line_numbers[1]
                .split(' ')
                .filter(|&s| !s.is_empty())
                .for_each(|s| {
                    if is_winding_number {
                        if s != "|" {
                            winning_numbers.push(s);
                        } else {
                            is_winding_number = false;
                        }
                    } else if winning_numbers.contains(&s) {
                        num += 1;
                    }
                });

            if num > 0 {
                ((num - 1) as f32).exp2() as u32
            } else {
                0
            }
        }).sum()

}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn day4_1_test() {
        let result = total_points(Path::new("src/day4/day4_input_test.txt"));
        assert_eq!(result, 13);
    }

    #[test]
    fn day4_1_answer() {
        let result = total_points(Path::new("src/day4/day4_input.txt"));
        assert_eq!(result, 533784);
    }

    #[test]
    fn day4_2_test() {
        let result = total_points(Path::new("src/day4/day4_input_test.txt"));
        assert_eq!(result, 467835);
    }

    #[test]
    fn day4_2_answer() {
        let result = total_points(Path::new("src/day4/day4_input.txt"));
        assert_eq!(result, 78826761);
    }
}
