use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::vec;
use crate::utils::read_lines;

fn format_data(file: &Path) -> Vec<Vec<i32>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let positions = strs.split('~').collect::<Vec<&str>>();
            let start: Vec<i32> = positions[0].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            let end: Vec<i32> = positions[1].split(',').map(|s| s.parse::<i32>().unwrap()).collect();
            data.push(vec![
                start[0],
                start[1],
                start[2],
                end[0],
                end[1],
                end[2],
            ])
        });
    data.sort_by(|a, d| a[2].cmp(&d[2]));
    data
}

fn overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    a[0].max(b[0]) <= a[3].min(b[3]) && a[1].max(b[1]) <= a[4].min(b[4])
}

fn get_bricks(bricks: &mut Vec<Vec<i32>>) -> u64 {
    for brick_i in 0..bricks.len() {
        let mut max_z = 1;
        for check_i in 0..brick_i {
            if overlap(&bricks[brick_i], &bricks[check_i]) {
                max_z = max_z.max(bricks[check_i][5] + 1);
            }
        }
        bricks[brick_i][5] -= bricks[brick_i][2] - max_z;
        bricks[brick_i][2] = max_z;
    }
    bricks.sort_by(|a, d| a[2].cmp(&d[2]));

    let mut k_supports_v = HashMap::new();
    for i in 0..bricks.len() {
        k_supports_v.insert(i, HashSet::new());
    }
    let mut v_supports_k = HashMap::new();
    for i in 0..bricks.len() {
        v_supports_k.insert(i, HashSet::new());
    }

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[..j].iter().enumerate() {
            if overlap(lower, upper) && upper[2] == lower[5] + 1 {
                k_supports_v.get_mut(&i).unwrap().insert(j);
                v_supports_k.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    let mut total = 0;
    for i in 0..bricks.len() {
        if k_supports_v[&i].iter().all(|j| v_supports_k[j].len() >= 2) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod day22_tests {
    use super::*;

    #[test]
    fn day22_1_test() {
        let mut data = format_data(Path::new("src/day22/day22_input_test.txt"));
        let result = get_bricks(&mut data);
        assert_eq!(result, 5);
    }

    #[test]
    fn day22_1_answer() {
        let mut data = format_data(Path::new("src/day22/day22_input.txt"));
        let result = get_bricks(&mut data);
        assert_eq!(result, 3841);
    }

    #[test]
    fn day22_2_test() {
        let mut data = format_data(Path::new("src/day22/day22_input_test.txt"));
        let result = get_bricks(&mut data);
        assert_eq!(result, 167004);
    }

    #[test]
    fn day22_2_answer() {
        let mut data = format_data(Path::new("src/day22/day22_input.txt"));
        let result = get_bricks(&mut data);
        assert_eq!(result, 128163929109524);
    }
}

