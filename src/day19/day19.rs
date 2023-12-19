use std::collections::HashMap;
use std::path::Path;
use std::vec;
use regex::Regex;
use crate::utils::read_lines;

fn format_data(file: &Path) -> (HashMap<String, Vec<(String, i64, i64, String)>>, Vec<HashMap<String, i64>>) {
    let mut workflows: HashMap<String, Vec<(String, i64, i64, String)>> = HashMap::new();
    let mut ratings = vec![];
    let mut first_part = true;
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            if strs.is_empty() {
                first_part = false;
            }
            if !strs.is_empty() {
                if first_part {
                    let re = Regex::new(r"(?P<pos>[a-z]+)\{(?P<text>[0-9a-zA-Z,:<>]+)}").unwrap();
                    let caps = re.captures(&strs).unwrap();
                    let key = caps["pos"].to_string();
                    let mut map = vec![];
                    let mut last_item = "";
                    let text = caps["text"].to_string();
                    text.split(",").for_each(|s| {
                        if !s.contains(":") {
                            last_item = s;
                        } else {
                            let dest = s.split(":").last().unwrap();
                            let first_part = s.split(":").next().unwrap();
                            let start = s.chars().next().unwrap();
                            let compare = if first_part.contains("<") { -1 } else { 1 };
                            let number = first_part[2..].parse::<i64>().unwrap();
                            map.push((start.to_string(), compare, number, dest.to_string()))
                        }
                    });
                    map.push(("".to_string(), 1, 1, last_item.to_string()));
                    workflows.insert(key, map);
                } else {
                    let re = Regex::new(r"\{(?P<text>[0-9a-z,=]+)}").unwrap();
                    let caps = re.captures(&strs).unwrap();
                    let mut rating = HashMap::new();
                    caps["text"].split(",")
                        .for_each(|s| {
                            let steps = s.split("=").collect::<Vec<&str>>();
                            rating.insert(steps[0].to_string(), steps[1].parse::<i64>().unwrap());
                        });
                    ratings.push(rating);
                }
            }
        });
    (workflows, ratings)
}

fn get_rating_numbers(workflows: HashMap<String, Vec<(String, i64, i64, String)>>, ratings: Vec<HashMap<String, i64>>) -> i64 {
    let mut result: i64 = 0;
    for rating in ratings {
        let mut start = "in";
        let mut idx = 0;
        loop {
            println!("start: {}", start);
            if start == "R" {
                break;
            } else if start == "A" {
                result += rating.values().sum::<i64>();
                break;
            }
            let workflow = workflows.get(start).unwrap();
            let next = workflow.get(idx).unwrap();
            // if its last item
            if next.0.is_empty() {
                start = &next.3;
                idx = 0;
            } else {
                let current_num = rating.get(&next.0).unwrap();
                // check if its rule is true
                if (current_num - next.2) * next.1 > 0 {
                    start = &next.3;
                    idx = 0;
                } else {
                    idx += 1;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    #[test]
    fn day19_1_test() {
        let (workflows, ratings) = format_data(Path::new("src/day19/day19_input_test.txt"));
        let result = get_rating_numbers(workflows, ratings);
        assert_eq!(result, 19114);
    }

    #[test]
    fn day19_1_answer() {
        let (workflows, ratings) = format_data(Path::new("src/day19/day19_input.txt"));
        let result = get_rating_numbers(workflows, ratings);
        assert_eq!(result, 391132);
    }

    #[test]
    fn day19_2_test() {
        let (workflows, ratings) = format_data(Path::new("src/day19/day19_input_test.txt"));
        let result = get_rating_numbers(workflows, ratings);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn day19_2_answer() {
        let (workflows, ratings) = format_data(Path::new("src/day19/day19_input.txt"));
        let result = get_rating_numbers(workflows, ratings);
        assert_eq!(result, 90111113594927);
    }
}
