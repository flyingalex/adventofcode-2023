use std::collections::{HashMap, HashSet};
use std::ops::Div;
use std::path::Path;
use std::vec;
use crate::utils::read_lines;

fn format_data(file: &Path) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut data = vec![];
    let mut start = (0, 0);
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let chars = strs.chars().collect::<Vec<char>>();
            if let Some(idx) = chars.iter().position(|&c| c == 'S') {
                start = (data.len() as i32, idx as i32);
            }
            data.push(strs.chars().collect::<Vec<char>>());
        });
    (data, start)
}

fn update_div_euclid(map: &mut HashMap<(i32, i32), HashSet<(i32, i32)>>, point: (i32, i32), x_len: i32, y_len: i32) {
    let x_div = point.0.div_euclid(x_len);
    let y_div = point.1.div_euclid(y_len);
    let real_x = point.0.rem_euclid(x_len);
    let real_y = point.1.rem_euclid(y_len);
    if (x_div, y_div) != (0, 0) {
        println!("x_div, y_div {} {}", x_div, y_div);
        if let Some(key) = map.get_mut(&(real_x, real_y)) {
            key.insert((x_div, y_div));
        } else {
            map.insert((real_x, real_y), HashSet::from([(x_div, y_div)]));
        }
    }
}

fn get_garden_plots(data: Vec<Vec<char>>, start: (i32, i32), step: i32) -> u64 {
    let mut steps = HashSet::new();
    steps.insert(start);
    let mut next_steps = HashSet::new();
    let mut next_steps_map: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut plots = HashSet::new();
    let x_len = data.len() as i32;
    let y_len = data[0].len() as i32;
    for i in 0..step {
        for (x, y) in steps {
            for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let nx = x + dx;
                let ny = y + dy;
                let real_x = nx.rem_euclid(x_len);
                let real_y = ny.rem_euclid(y_len);
                if data[real_x as usize][real_y as usize] == '.' || data[real_x as usize][real_y as usize] == 'S' {
                    next_steps.insert((real_x, real_y));
                    if i == step - 1 {
                        plots.insert((real_x, real_y));
                    }
                    update_div_euclid(&mut next_steps_map, (nx, ny), x_len, y_len);
                }
            }
        }
        steps = next_steps;
        next_steps = HashSet::new();
    }
    next_steps_map.values().map(|v| v.len() as i32).sum::<i32>() as u64
    + plots.iter().filter(|p| next_steps_map.contains_key(p)).count() as u64
}

#[cfg(test)]
mod day21_tests {
    use super::*;

    #[test]
    fn day21_1_test() {
        let (data, start) = format_data(Path::new("src/day21/day21_input_test.txt"));
        let result = get_garden_plots(data, start, 6);
        assert_eq!(result, 16);
    }

    #[test]
    fn day21_1_answer() {
        let (data, start) = format_data(Path::new("src/day21/day21_input.txt"));
        let result = get_garden_plots(data, start, 64);
        assert_eq!(result, 3841);
    }

    #[test]
    fn day21_2_test() {
        let (data, start) = format_data(Path::new("src/day21/day21_input_test.txt"));
        let result = get_garden_plots(data, start, 26501365);
        assert_eq!(result, 167004);
    }

    #[test]
    fn day21_2_answer() {
        let (data, start) = format_data(Path::new("src/day21/day21_input.txt"));
        let result = get_garden_plots(data, start, 12);
        assert_eq!(result, 128163929109524);
    }
}

