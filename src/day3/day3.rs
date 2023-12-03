use std::path::Path;
use crate::utils::read_lines;

fn calculate_number(engine_schematic: &Vec<Vec<char>>) -> u32 {
    let mut result = 0;
    let mut num = vec![];
    for (pos, c) in engine_schematic[1].iter().enumerate() {
        if c.is_ascii_digit() {
            num.push((pos, c));
        } else if !num.is_empty() {
            // collect all adjacent elements
            let mut adjacent: Vec<char> = vec![];
            // first row
            adjacent.append(&mut engine_schematic[0][num[0].0 - 1..=pos].to_vec());
            // last row
            adjacent.append(&mut engine_schematic[2][num[0].0 - 1..=pos].to_vec());
            // middle row's before and after element
            adjacent.push(engine_schematic[1][num[0].0 - 1]);
            adjacent.push(engine_schematic[1][pos]);
            for c in adjacent {
                // check if there is symbol
                if !c.is_ascii_digit() && c != '.' {
                    // add to result
                    result += num.iter().map(|(_, &c)| c).collect::<String>().parse::<u32>().unwrap();
                    break;
                }
            }
            num = vec![];
        }
    }
    println!("result is {}", result);
    result
}

fn sum_part_numbers(file: &Path) -> u32 {
    let mut result = 0;
    let mut engine_schematic: Vec<Vec<char>> = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            if engine_schematic.len() < 3 {
                let strs = l.unwrap();
                let mut sub_strs = strs.chars().collect::<Vec<char>>();
                sub_strs.push('.');
                sub_strs.insert(0, '.');

                // add a new '.' row to make data always has same structure
                if engine_schematic.is_empty() {
                    engine_schematic = vec![vec!['.'; sub_strs.len()]]
                }

                engine_schematic.push(sub_strs);
            }

            if engine_schematic.len() == 3 {
                result += calculate_number(engine_schematic.as_mut());

                // remove first 2 elements
                engine_schematic.remove(0);
            }
        });

    // last 2 lines
    engine_schematic.push(vec!['.'; engine_schematic[0].len()]);
    result += calculate_number(engine_schematic.as_mut());

    result
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn day3_1_test() {
        let result = sum_part_numbers(Path::new("src/day3/day3_input_test.txt"));
        assert_eq!(result, 4361);
    }

    #[test]
    fn day3_1_answer() {
        let result = sum_part_numbers(Path::new("src/day3/day3_input.txt"));
        assert_eq!(result, 533784);
    }

    #[test]
    fn day3_2_test() {
        let result = sum_part_numbers(Path::new("src/day3/day3_input_test.txt"));
        assert_eq!(result, 2286);
    }

    #[test]
    fn day3_2_answer() {
        let result = sum_part_numbers(Path::new("src/day3/day3_input.txt"));
        assert_eq!(result, 60948);
    }
}
