use std::path::Path;
use crate::utils::read_lines;

fn found_next_location(location: u64, vec_map: &Vec<Vec<u64>>) -> u64 {
    for vec_map_item in vec_map {
        if vec_map_item[1] <= location && location <= (vec_map_item[1] + vec_map_item[2]) {
            return vec_map_item[0] + (location - vec_map_item[1]);
        }
    }
    location
}

fn find_lowest_location_number(file: &Path) -> u64 {
    let line = read_lines(file).unwrap().next().unwrap().unwrap();
    let numbers = line.split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let mut locations = vec![];
    for number in numbers {
        let mut start_count = false;
        let mut vec_map: Vec<Vec<u64>> = vec![];
        let mut location = number;
        read_lines(file)
            .unwrap()
            .for_each(|l| {
                let strs = l.unwrap();
                if strs.contains("map") {
                    start_count = true;
                }
                if start_count && !strs.is_empty() && !strs.contains("map") {
                    let line_numbers = strs.split(' ')
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect::<Vec<u64>>();
                    vec_map.push(line_numbers);
                }
                if strs.is_empty() && !vec_map.is_empty() {
                    location = found_next_location(location, &vec_map);
                    vec_map = vec![];
                    start_count = false;
                }
                // first line, get all seeds
            });
        // calculate last map
        location = found_next_location(location, &vec_map);
        locations.push(location);
    }

    locations.sort();
    *(locations.first().unwrap())
}

fn format_data(file: &Path) -> (Vec<(u64, u64)>, Vec<Vec<(u64, u64, u64)>>) {
    let mut seeds = vec![];

    let mut first = true;
    let mut seed_to_soil = vec![];
    let mut start_count = false;
    let mut vec_map = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            if first {
                seeds = strs.split(' ')
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect::<Vec<u64>>().chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
                first = false;
            } else {
                if strs.contains("map") {
                    start_count = true;
                }
                if start_count && !strs.is_empty() && !strs.contains("map") {
                    let line_numbers = strs.split(' ')
                        .filter_map(|s| s.parse::<u64>().ok())
                        .collect::<Vec<u64>>();
                    vec_map.push((line_numbers[0], line_numbers[1], line_numbers[2]));
                }
                if strs.is_empty() {
                    if !vec_map.is_empty() {
                        seed_to_soil.push(vec_map.clone());
                    }
                    vec_map = vec![];
                    start_count = false;
                }
            }
        });
    seed_to_soil.push(vec_map);

    (seeds, seed_to_soil)
}

fn find_lowest_location_number2(seeds: Vec<(u64, u64)>, seed_to_soils: Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut loop_seeds = seeds.iter().map(|&s| (s.0, s.1 + s.0)).collect::<Vec<(u64, u64)>>();
    // calculate each block
    for seed_to_soil in seed_to_soils.iter() {
        let mut new_seeds = vec![];
        while !loop_seeds.is_empty() {
            let (start, end) = loop_seeds.remove(0);
            let mut has_break = false;
            for &(destination, source_start, source_len) in seed_to_soil {
                // check overlap s and e
                let overlap_start = start.max(source_start);
                let overlap_end = end.min(source_start + source_len);
                if overlap_start < overlap_end {
                    new_seeds.push((overlap_start - source_start + destination, overlap_end - source_start + destination));
                    if overlap_start > start {
                        loop_seeds.push((start, overlap_start));
                    }
                    if end > overlap_end {
                        loop_seeds.push((overlap_end, end));
                    }
                    has_break = true;
                    break;
                }
            }
            // if there is no overlap with seed_to_soil, then put in the new_seeds
            if !has_break {
                new_seeds.push((start, end));
            }
        }
        loop_seeds = new_seeds;
    }
    loop_seeds.iter().min().unwrap().0
}


#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn day5_1_test() {
        let result = find_lowest_location_number(Path::new("src/day5/day5_input_test.txt"));
        assert_eq!(result, 35);
    }

    #[test]
    fn day5_1_answer() {
        let result = find_lowest_location_number(Path::new("src/day5/day5_input.txt"));
        assert_eq!(result, 806029445);
    }

    #[test]
    fn day5_2_test() {
        let (seeds, seed_to_soil) = format_data(Path::new("src/day5/day5_input_test.txt"));
        let result = find_lowest_location_number2(seeds, seed_to_soil);
        assert_eq!(result, 46);
    }

    #[test]
    fn day5_2_answer() {
        let (seeds, seed_to_soil) = format_data(Path::new("src/day5/day5_input.txt"));
        let result = find_lowest_location_number2(seeds, seed_to_soil);
        assert_eq!(result, 59370572);
    }
}
