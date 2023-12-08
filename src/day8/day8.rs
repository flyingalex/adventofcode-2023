use std::collections::HashMap;
use std::path::Path;
use regex::Regex;
use crate::utils::read_lines;

#[derive(Debug)]
struct Hand {
    strength: String,
    bid: u64,
    card_map: HashMap<char, u64>,
}

fn format_data(file: &Path) -> (Vec<u64>, HashMap<String, Vec<String>>) {
    let mut directions = vec![];
    let mut route_map = HashMap::new();
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            if directions.is_empty() {
                let camel_card = strs.split("").filter(|s| !s.is_empty()).map(|s| {
                    if s.contains('R') {
                        1
                    } else {
                        0
                    }
                }).collect();
                directions = camel_card;
            } else if !strs.is_empty() {
                let re = Regex::new(r"(?P<start>[A-Z]+) = \((?P<left>[A-Z]+), (?P<right>[A-Z]+)\)").unwrap();
                let caps = re.captures(&strs).unwrap();
                route_map.insert(caps["start"].to_string(), vec![
                    caps["left"].to_string(),
                    caps["right"].to_string(),
                ]);
            }
        });

    (directions, route_map)
}

// 1, 2, 2,  3, 3, 4, 5
fn total_winnings(directions: &Vec<u64>, route_map: &HashMap<String, Vec<String>>) -> u64 {
    let mut count = 0;
    let mut start_point = "AAA";
    loop {
        for d in directions {
            start_point = route_map.get(start_point).unwrap().get(*d as usize).unwrap();
            count += 1;
            if start_point == "ZZZ" {
                break;
            }
        }
        if start_point == "ZZZ" {
            break;
        }
    }
    count
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn day8_1_test() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input_test.txt"));
        let result = total_winnings(&directions, &route_map);
        assert_eq!(result, 6);
    }

    #[test]
    fn day8_1_answer() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input.txt"));
        let result = total_winnings(&directions, &route_map);
        assert_eq!(result, 11309);
    }

    #[test]
    fn day8_2_test() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input_test.txt"));
        let result = total_winnings(&directions, &route_map);
        assert_eq!(result, 5905);
    }

    #[test]
    fn day8_2_answer() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input.txt"));
        let result = total_winnings(&directions, &route_map);
        assert_eq!(result, 249356515);
    }
}
