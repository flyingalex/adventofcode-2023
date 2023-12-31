use std::collections::HashMap;
use std::path::Path;
use regex::Regex;
use crate::utils::read_lines;

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
                let re = Regex::new(r"(?P<start>[0-9A-Z]+) = \((?P<left>[0-9A-Z]+), (?P<right>[0-9A-Z]+)\)").unwrap();
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
fn get_steps(directions: &Vec<u64>, route_map: &HashMap<String, Vec<String>>) -> u64 {
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

fn get_steps2(directions: &Vec<u64>, route_map: &HashMap<String, Vec<String>>) -> u64 {
    let start_points = route_map.keys().filter(|r| r.ends_with('A')).collect::<Vec<&String>>();
    let mut steps_to_end = vec![0; start_points.len()];
    for (idx, &start_point) in start_points.iter().enumerate() {
        let mut count = 0;
        let mut new_start_point = start_point;
        loop {
            for d in directions {
                count += 1;
                new_start_point = route_map.get(new_start_point).unwrap().get(*d as usize).unwrap();
                if new_start_point.ends_with('Z') {
                    steps_to_end[idx] = count;
                    break;
                }
            }
            if new_start_point.ends_with('Z') {
                break;
            }
        }

        if steps_to_end.iter().all(|&s| s > 0) {
            break;
        }
    }

    // get Least common multiple
    steps_to_end.iter().fold(steps_to_end[0], |acc, &x| num::integer::lcm(acc, x))
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn day8_1_test() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input_test.txt"));
        let result = get_steps(&directions, &route_map);
        assert_eq!(result, 6);
    }

    #[test]
    fn day8_1_answer() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input.txt"));
        let result = get_steps(&directions, &route_map);
        assert_eq!(result, 11309);
    }

    #[test]
    fn day8_2_test() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input_test2.txt"));
        let result = get_steps2(&directions, &route_map);
        assert_eq!(result, 6);
    }

    #[test]
    fn day8_2_answer() {
        let (directions, route_map) = format_data(Path::new("src/day8/day8_input.txt"));
        let result = get_steps2(&directions, &route_map);
        assert_eq!(result, 13740108158591);
    }
}
