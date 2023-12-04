use std::path::Path;
use crate::utils::read_lines;

fn each_line_winner_count(line: &str) -> u32 {
    let mut winning_numbers: Vec<&str> = vec![];
    let mut is_winning_number = true;
    let mut num = 0;
    line
        .split(' ')
        .filter(|&s| !s.is_empty())
        .for_each(|s| {
            if is_winning_number {
                if s != "|" {
                    winning_numbers.push(s);
                } else {
                    is_winning_number = false;
                }
            } else if winning_numbers.contains(&s) {
                num += 1;
            }
        });
    num
}

fn total_points(file: &Path) -> u32 {
    read_lines(file)
        .unwrap()
        .map(|l| {
            let strs = l.unwrap();
            let line_numbers = strs.split(':').collect::<Vec<&str>>();
            let num = each_line_winner_count(line_numbers[1]);
            if num > 0 {
                ((num - 1) as f32).exp2() as u32
            } else {
                0
            }
        }).sum()
}

fn total_points2(file: &Path) -> u32 {
    let mut winning_rows = read_lines(file)
        .unwrap()
        .map(|l| {
            let strs = l.unwrap();
            let line_numbers = strs.split(':').collect::<Vec<&str>>();
            let num = each_line_winner_count(line_numbers[1]);
            // return each line win how many copies
            vec![num]
        })
        .collect::<Vec<Vec<u32>>>();

    for pos in 0..winning_rows.len() {
        let num = winning_rows[pos][0];
        // every item need to loop one time
        for _ in 0..winning_rows[pos].len() {
            for i in (pos + 1)..=(pos + (num as usize)) {
                let same_number = winning_rows[i][0];
                winning_rows[i].push(same_number);
            }
        }
    }

    winning_rows.iter().map(|w| {
        w.len() as u32
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
        assert_eq!(result, 26443);
    }

    #[test]
    fn day4_2_test() {
        let result = total_points2(Path::new("src/day4/day4_input_test.txt"));
        assert_eq!(result, 30);
    }

    #[test]
    fn day4_2_answer() {
        let result = total_points2(Path::new("src/day4/day4_input.txt"));
        assert_eq!(result, 6284877);
    }
}
