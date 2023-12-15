use std::collections::HashMap;
use std::path::Path;
use std::process::id;
use itertools::Itertools;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<Vec<char>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let s = strs.split(',').collect::<Vec<&str>>();
            let springs = s
                .iter()
                .map(|&s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            data = springs;
        });
    data
}

fn get_result_of_sequence(data: Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    for datum in data {
        let mut start = 0;
        datum.iter().for_each(|&c| {
            start = ((start + c as u64) * 17) % 256;
        });
        result += start;
    }
    result
}

fn get_result_of_sequence2(data: Vec<Vec<char>>) -> u64 {
    let mut map: HashMap<u64, Vec<(String, u64)>> = HashMap::new();
    let mut len_pos: HashMap<String, u64> = HashMap::new();
    for datum in data {
        // HASH to find box idx
        let mut new_idx = 0;
        let mut label = String::new();
        for (datum_idx, &c) in datum.iter().enumerate() {
            if c == '-' {
                // remove label in box if found
                if let Some(idx) = len_pos.get(label.as_str()) {
                    let mut found_pos_lens = map.get_mut(idx).unwrap();
                    let index = found_pos_lens.iter().position(|(cur_label, _)| *cur_label == label).unwrap();
                    found_pos_lens.remove(index);
                    len_pos.remove(label.as_str());
                }
                break;
            }

            if c == '=' {
                // update label length in box if found
                let num = datum[datum_idx + 1].to_digit(10).unwrap() as u64;
                if let Some(idx) = len_pos.get(label.as_str()) {
                    let mut found_pos_lens = map.get_mut(idx).unwrap();
                    let index = found_pos_lens.iter().position(|(cur_label, _)| *cur_label == label).unwrap();
                    found_pos_lens[index].1 = num;
                } else {
                    // or add label in box if not found
                    if let Some(len) = map.get_mut(&new_idx) {
                        len.push((label.clone(), num));
                    } else {
                        map.insert(new_idx, vec![(label.clone(), num)]);
                    }
                    len_pos.insert(label, new_idx);
                }
                break;
            }

            new_idx = ((new_idx + c as u64) * 17) % 256;
            label += c.to_string().as_str();
        }
    }

    map.iter().map(|(box_idx, pos_lens)| {
        pos_lens.iter().enumerate().map(|(slot_idx, (_, len))| {
            (box_idx + 1) * (slot_idx + 1) as u64 * len
        }).sum::<u64>()
    }).sum()
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    #[test]
    fn day15_1_test() {
        let data = format_data(Path::new("src/day15/day15_input_test.txt"));
        let result = get_result_of_sequence(data);
        assert_eq!(result, 136);
    }

    #[test]
    fn day15_1_answer() {
        let data = format_data(Path::new("src/day15/day15_input.txt"));
        let result = get_result_of_sequence(data);
        assert_eq!(result, 514639);
    }

    #[test]
    fn day15_2_test() {
        let data = format_data(Path::new("src/day15/day15_input_test.txt"));
        let result = get_result_of_sequence2(data);
        assert_eq!(result, 145);
    }

    #[test]
    fn day15_2_answer() {
        let data = format_data(Path::new("src/day15/day15_input.txt"));
        let result = get_result_of_sequence2(data);
        assert_eq!(result, 90551);
    }
}
