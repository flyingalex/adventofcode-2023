use std::collections::HashMap;
use std::path::Path;
use std::vec;
use regex::Regex;
use crate::utils::read_lines;


fn format_data(file: &Path) -> Vec<(String, i64)> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let re = Regex::new(r"(?P<dir>[RLDU]) (?P<count>[0-9]+) \((?P<color>#[0-9a-z]+)\)").unwrap();
            let caps = re.captures(&strs).unwrap();
            data.push((
                caps["dir"].to_string(),
                caps["count"].to_string().parse::<i64>().unwrap()
            ));
        });
    data
}

fn format_data2(file: &Path) -> Vec<(String, i64)> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let re = Regex::new(r"\(#(?P<hex>[0-9a-z]{5})(?P<dir>[0123])\)").unwrap();
            let caps = re.captures(&strs).unwrap();
            let dir_num = caps["dir"].to_string().parse::<i64>().unwrap();
            let dir = HashMap::from([
                (0, "R"),
                (1, "D"),
                (2, "L"),
                (3, "U"),
            ]);

            data.push((
                dir.get(&dir_num).unwrap().to_string(),
                i64::from_str_radix(caps["hex"].to_string().as_str(), 16).unwrap(),
            ));
        });
    data
}

// The Shoelace Algorithm
fn shoelace_formula(v: Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    for pos in v.windows(2) {
        let first = pos[0];
        let second = pos[1];
        sum += first.0 * second.1 - first.1 * second.0;
    }

    sum.abs() / 2
}

fn get_cubic_meters(data: Vec<(String, i64)>) -> i64 {
    let directions: HashMap<&str, (i64, i64)> = HashMap::from([
        ("U", (-1, 0)),
        ("R", (0, 1)),
        ("D", (1, 0)),
        ("L", (0, -1)),
    ]);
    let mut terrain: HashMap<(i64, i64), bool> = HashMap::new();
    let mut current = (0, 0);
    terrain.insert(current, true);
    let mut vertices = vec![(0, 0)];
    let mut terrain_count = 1;
    data.iter().for_each(|(dir, count)| {
        let &(x, y) = directions.get(dir.as_str()).unwrap();
        terrain_count += *count;
        current = (current.0 + x * *count, current.1 + y * *count);
        vertices.push(current);
    });
    vertices.push((0, 0));

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    shoelace_formula(vertices) + terrain_count / 2 + 1
}

#[cfg(test)]
mod day18_tests {
    use super::*;

    #[test]
    fn day18_1_test() {
        let data = format_data(Path::new("src/day18/day18_input_test.txt"));
        let result = get_cubic_meters(data);
        assert_eq!(result, 62);
    }

    #[test]
    fn day18_1_answer() {
        let data = format_data(Path::new("src/day18/day18_input.txt"));
        let result = get_cubic_meters(data);
        assert_eq!(result, 40745);
    }

    #[test]
    fn day18_2_test() {
        let data = format_data2(Path::new("src/day18/day18_input_test.txt"));
        let result = get_cubic_meters(data);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn day18_2_answer() {
        let data = format_data2(Path::new("src/day18/day18_input.txt"));
        let result = get_cubic_meters(data);
        assert_eq!(result, 90111113594927);
    }
}
