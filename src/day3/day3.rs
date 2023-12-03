use std::path::Path;
use crate::utils::read_lines;

fn calculate_number(engine_schematic: &[Vec<char>]) -> u32 {
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

fn calculate_line_gear(line: &[char], pos: usize) -> Vec<u32> {
    // collect all adjacent elements
    let mut adjacent: Vec<u32> = vec![];

    let mut pre_chars = vec![];
    let mut pre_idx = pos - 1;
    loop {
        if line[pre_idx].is_ascii_digit() {
            pre_chars.insert(0, line[pre_idx]);
            pre_idx -= 1;
        } else {
            break;
        }
    }
        // adjacent.push(num_chars.iter().collect::<String>().parse::<u32>().unwrap());

    let mut next_chars = vec![];
    let mut next_idx = pos + 1;
    loop {
        if line[next_idx].is_ascii_digit() {
            next_chars.push(line[next_idx]);
            next_idx += 1;
        } else {
            break;
        }
    }

    // if center is digit, then connect it
    if line[pos].is_ascii_digit() {
        pre_chars.append(vec![line[pos]].as_mut());
        pre_chars.append(&mut next_chars);
        adjacent.push(pre_chars.iter().collect::<String>().parse::<u32>().unwrap());
    } else {
        // otherwise add them separately
        if !pre_chars.is_empty() {
            adjacent.push(pre_chars.iter().collect::<String>().parse::<u32>().unwrap());
        }
        if !next_chars.is_empty() {
            adjacent.push(next_chars.iter().collect::<String>().parse::<u32>().unwrap());
        }
    }
    adjacent
}

fn calculate_gear_ratios(engine_schematic: &[Vec<char>]) -> u32 {
    let mut num = vec![];
    for (pos, &c) in engine_schematic[1].iter().enumerate() {
        if c == '*' {
            let mut single_gears = vec![];
            // collect all adjacent elements, use 3 rows as one calulation unit
            for idx in [0, 1, 2] {
                single_gears.append(&mut calculate_line_gear(&engine_schematic[idx], pos));
            }
            if single_gears.len() == 2 {
                num.push(single_gears[0] * single_gears[1]);
            }
        }
    }
    num.iter().sum()
}

fn puzzle_answer(file: &Path, calculate_fn: &dyn Fn(&[Vec<char>]) -> u32) -> u32 {
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
                result += calculate_fn(engine_schematic.as_mut());

                // remove first 2 elements
                engine_schematic.remove(0);
            }
        });

    // add one line for last 2 lines
    engine_schematic.push(vec!['.'; engine_schematic[0].len()]);
    result += calculate_fn(engine_schematic.as_mut());

    result
}

#[cfg(test)]
mod day3_tests {
    use super::*;

    #[test]
    fn day3_1_test() {
        let result = puzzle_answer(Path::new("src/day3/day3_input_test.txt"), &calculate_number);
        assert_eq!(result, 4361);
    }

    #[test]
    fn day3_1_answer() {
        let result = puzzle_answer(Path::new("src/day3/day3_input.txt"), &calculate_number);
        assert_eq!(result, 533784);
    }

    #[test]
    fn day3_2_test() {
        let result = puzzle_answer(Path::new("src/day3/day3_input_test.txt"), &calculate_gear_ratios);
        assert_eq!(result, 467835);
    }

    #[test]
    fn day3_2_answer() {
        let result = puzzle_answer(Path::new("src/day3/day3_input.txt"), &calculate_gear_ratios);
        assert_eq!(result, 78826761);
    }
}
