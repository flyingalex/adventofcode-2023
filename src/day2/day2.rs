use std::collections::HashMap;
use std::path::Path;
use std::ptr::hash;
use crate::utils::read_lines;

fn sum_games_id(file: &Path) -> u32 {
    let cubes_in_bag = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);
    read_lines(file)
        .unwrap()
        .map(|l| {
            let strs = l.unwrap();
            let sub_strs = strs.split(&[':', ';'][..]).collect::<Vec<&str>>();
            let mut id = 0;
            sub_strs.iter().for_each(|&s| {
                if s.contains("Game") {
                    id = s.split(' ').last().unwrap().parse::<u32>().unwrap();
                } else {
                    s
                        .split(&[' ', ','][..])
                        .filter(|&s| !s.is_empty())
                        .collect::<Vec<&str>>()
                        .chunks(2)
                        .for_each(|w| {
                            let num = w[0].parse::<i32>().unwrap();
                            if let Some(&total) = cubes_in_bag.get(w[1]) && total < num {
                                id = 0;
                            }
                        })
                }
            });
            id
        })
        .sum()
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn day2_1_test() {
        let result = sum_games_id(Path::new("src/day2/day2_input_test1.txt"));
        assert_eq!(result, 8);
    }

    #[test]
    fn day2_1_answer() {
        let result = sum_games_id(Path::new("src/day2/day2_input.txt"));
        assert_eq!(result, 2169);
    }

    #[test]
    fn day2_2_test() {
        let result = sum_games_id(Path::new("src/day2/day2_input_test2.txt"));
        assert_eq!(result, 281);
    }

    #[test]
    fn day2_2_answer() {
        let result = sum_games_id(Path::new("src/day2/day2_input.txt"));
        assert_eq!(result, 53592);
    }
}
