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
        let result = find_lowest_location_number(Path::new("src/day5/day5_input_test.txt"));
        assert_eq!(result, 30);
    }

    #[test]
    fn day5_2_answer() {
        let result = find_lowest_location_number(Path::new("src/day5/day5_input.txt"));
        assert_eq!(result, 6284877);
    }
}
