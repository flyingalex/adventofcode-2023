use std::path::Path;
use itertools::Itertools;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<(Vec<char>, Vec<u32>)> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let s = strs.split(' ').collect::<Vec<&str>>();
            let mut springs = vec![];
            let mut rows = vec![];
            springs = s[0].chars().collect::<Vec<char>>();
            rows = s[1].split(',').map(|c| c.parse::<u32>().unwrap()).collect();
            data.push((springs, rows));
        });
    // check if all spaces in one column is *
    data
}

fn damage_idxs_same_with_rows(idxs: &Vec<usize>, rows: &Vec<u32>) -> bool {
    let mut counts = vec![];
    // split to groups
    let mut idx_count = 1;
    let mut idxs_cloned = idxs.clone();
    idxs_cloned.sort();
    idxs_cloned.windows(2).for_each(|w| {
        if w[0] + 1 == w[1] {
            idx_count += 1;
        } else {
            counts.push(idx_count as u32);
            idx_count = 1;
        }
    });
    if idx_count > 0 {
        counts.push(idx_count as u32);
    }

    counts == *rows
}

fn get_arrangement_sum(data: Vec<(Vec<char>, Vec<u32>)>) -> u64 {
    let mut counts = vec![];
    for datum in data {
        let mut damaged_idx = vec![];
        let mut unknown_idx = vec![];
        // collect damaged and unknown idx
        datum.0.iter().enumerate().for_each(|(idx, c)| {
            if c == &'#' {
                damaged_idx.push(idx);
            }
            if c == &'?' {
                unknown_idx.push(idx);
            }
        });

        if (datum.1.iter().sum::<u32>() - damaged_idx.len() as u32) <= unknown_idx.len() as u32 {
            let mut count = 0;
            let missed_items = datum.1.iter().sum::<u32>() as usize - damaged_idx.len();
            if missed_items > 0 {
                unknown_idx.iter().combinations(missed_items).for_each(|comb| {
                    let mut new_damaged_idx = damaged_idx.clone();
                    comb.iter().for_each(|&c| {
                        new_damaged_idx.push(*c);
                    });

                    if damage_idxs_same_with_rows(&new_damaged_idx, &datum.1) {
                        count += 1;
                    }
                });
            } else if damage_idxs_same_with_rows(&damaged_idx, &datum.1)
            {
                count += 1;
            }

            if count > 0 {
                counts.push(count);
            }
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    #[test]
    fn day12_1_test() {
        let data = format_data(Path::new("src/day12/day12_input_test.txt"));
        let result = get_arrangement_sum(data);
        assert_eq!(result, 21);
    }

    #[test]
    fn day12_1_answer() {
        let data = format_data(Path::new("src/day12/day12_input.txt"));
        let result = get_arrangement_sum(data);
        assert_eq!(result, 7705);
    }

    #[test]
    fn day12_2_test() {
        let data = format_data(Path::new("src/day12/day12_input_test.txt"));
        let result = get_arrangement_sum(data);
        assert_eq!(result, 1030);
    }

    #[test]
    fn day12_2_answer() {
        let data = format_data(Path::new("src/day12/day12_input.txt"));
        let result = get_arrangement_sum(data);
        assert_eq!(result, 305);
    }
}
