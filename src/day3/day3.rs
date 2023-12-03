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

fn calculate_line_gear(line: &Vec<char>, pos: usize) -> Vec<u32> {
    // collect all adjacent elements
    let mut adjacent: Vec<u32> = vec![];
    // first row
    // engine_schematic[0][pos-1] engine_schematic[0][pos] engine_schematic[0][pos+1]
    if line[pos].is_ascii_digit() {
        let mut num_chars = vec![line[pos]];
        let mut pre_idx = pos - 1;
        let mut next_idx = pos + 1;
        loop {
            if line[pre_idx].is_ascii_digit() {
                num_chars.insert(0, line[pre_idx]);
                pre_idx -= 1;
            }
            if line[next_idx].is_ascii_digit() {
                num_chars.push(line[next_idx]);
                next_idx += 1;
            }

            if !line[pre_idx].is_ascii_digit() && !line[next_idx].is_ascii_digit() {
                break
            }
        }
        adjacent.push(num_chars.iter().collect::<String>().parse::<u32>().unwrap());
    } else {
        let mut num_chars = vec![];
        let mut pre_idx = pos - 1;
        loop {
            if line[pre_idx].is_ascii_digit() {
                num_chars.insert(0, line[pre_idx]);
                pre_idx -= 1;
            } else {
                break;
            }
        }
        if !num_chars.is_empty() {
            adjacent.push(num_chars.iter().collect::<String>().parse::<u32>().unwrap());
        }

        num_chars = vec![];
        let mut next_idx = pos + 1;
        loop {
            if line[next_idx].is_ascii_digit() {
                num_chars.push(line[next_idx]);
                next_idx += 1;
            } else {
                break;
            }
        }
        if !num_chars.is_empty() {
            adjacent.push(num_chars.iter().collect::<String>().parse::<u32>().unwrap());
        }
    }
    adjacent
}

fn calculate_gear_ratios(engine_schematic: &Vec<Vec<char>>) -> u32 {
    let mut num = vec![];
    for (pos, &c) in engine_schematic[1].iter().enumerate() {
        if c == '*' {
            let mut single_gears = vec![];
            single_gears.append(&mut calculate_line_gear(&engine_schematic[0], pos));
            single_gears.append(&mut calculate_line_gear(&engine_schematic[1], pos));
            single_gears.append(&mut calculate_line_gear(&engine_schematic[2], pos));
            if single_gears.len() == 2 {
                num.push(single_gears[0] * single_gears[1]);
            }
        }
    }
    num.iter().sum()
}

fn sum_gear_ratio(file: &Path) -> u32 {
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
                result += calculate_gear_ratios(engine_schematic.as_mut());

                // remove first 2 elements
                engine_schematic.remove(0);
            }
        });

    // last 2 lines
    engine_schematic.push(vec!['.'; engine_schematic[0].len()]);
    result += calculate_gear_ratios(engine_schematic.as_mut());

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
        let result = sum_gear_ratio(Path::new("src/day3/day3_input_test.txt"));
        assert_eq!(result, 467835);
    }

    #[test]
    fn day3_2_answer() {
        let result = sum_gear_ratio(Path::new("src/day3/day3_input.txt"));
        assert_eq!(result, 78826761);
    }
}
