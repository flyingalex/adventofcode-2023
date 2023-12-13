use std::collections::HashMap;
use std::path::Path;
use crate::utils::read_lines;

fn format_data(file: &Path, step2: bool) -> Vec<(Vec<char>, Vec<u64>)> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let s = strs.split(' ').collect::<Vec<&str>>();
            let mut springs = s[0].chars().collect::<Vec<char>>();
            let mut rows: Vec<u64> = s[1].split(',').map(|c| c.parse::<u64>().unwrap()).collect();

            if step2 {
                let springs_cloned = springs.clone();
                springs.extend(vec!['?']);
                springs.extend(springs_cloned.clone());
                springs.extend(vec!['?']);
                springs.extend(springs_cloned.clone());
                springs.extend(vec!['?']);
                springs.extend(springs_cloned.clone());
                springs.extend(vec!['?']);
                springs.extend(springs_cloned.clone());
                let rows_cloned = rows.clone();
                rows.extend(rows_cloned.clone());
                rows.extend(rows_cloned.clone());
                rows.extend(rows_cloned.clone());
                rows.extend(rows_cloned.clone());
            }
            data.push((springs, rows));
        });
    // check if all spaces in one column is *
    data
}

fn calculate_arrangements(springs: Vec<char>, rows: &Vec<u64>, cache: &mut HashMap<String, u64>) -> u64 {
    let s_springs = springs.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("");
    let s_rows = rows.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("");
    let key = s_springs + &*s_rows;
    if cache.get(&key).is_some() {
        return *cache.get(&key).unwrap();
    }

    let mut count = 0;
    let current_range_len = rows[0] as usize;
    for (idx, s) in springs.windows(current_range_len).enumerate() {
        let current_len = idx + current_range_len;
        let has_number_before_idx = springs[0..idx].iter().any(|&x| x == '#');
        if has_number_before_idx {
            break;
        }
        if s.iter().all(|c| c != &'.') {
            // next position must be ./?, then go into this clause
            if rows.len() > 1 && springs.len() > current_len && springs[current_len] != '#' {
                count += calculate_arrangements(springs[current_len + 1..].to_vec(), &rows[1..].to_vec(), cache);
            }
            if rows.len() == 1 && springs[current_len..].iter().all(|&x| x != '#') {
                count += 1;
            }
        }
    }
    // println!("springs {:?}", count);
    cache.insert(key, count);
    count
}

fn get_arrangement_sum(data: Vec<(Vec<char>, Vec<u64>)>) -> u64 {
    let mut count = 0;
    data.iter().for_each(|(springs, rows)| {
        let mut cache: HashMap<String, u64> = HashMap::new();
        count += calculate_arrangements(springs.clone(), rows, &mut cache);
    });
    count
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn day12_1_test() {
        let data = format_data(Path::new("src/day12/day12_input_test.txt"), false);
        let result = get_arrangement_sum(data);
        assert_eq!(result, 21);
    }

    #[test]
    fn day12_1_answer() {
        let data = format_data(Path::new("src/day12/day12_input.txt"), false);
        let result = get_arrangement_sum(data);
        assert_eq!(result, 7705);
    }

    #[test]
    fn day12_2_test() {
        let data = format_data(Path::new("src/day12/day12_input_test.txt"), true);
        let result = get_arrangement_sum(data);
        assert_eq!(result, 525152);
    }

    #[test]
    fn day12_2_answer() {
        let data = format_data(Path::new("src/day12/day12_input.txt"), true);
        let result = get_arrangement_sum(data);
        assert_eq!(result, 50338344809230);
    }
}
