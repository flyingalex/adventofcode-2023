use std::path::Path;
use std::vec;
use crate::utils::read_lines;

/*
ref:
    https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day24p1.py
    https://www.youtube.com/watch?v=guOyA7Ijqgk
 */

struct Hailstone {
    sx: i128,
    sy: i128,
    _sz: i128,
    vx: i128,
    vy: i128,
    _vz: i128,
    a: i128,
    b: i128,
    c: i128,
}

impl Hailstone {
    fn new(sx: i128, sy: i128, _sz: i128, vx: i128, vy: i128, _vz: i128) -> Hailstone {
        let a = vy;
        let b = -vx;
        let c = vy * sx - vx * sy;
        Hailstone { sx, sy, _sz, vx, vy, _vz, a, b, c }
    }
}

fn format_data(file: &Path) -> Vec<Hailstone> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let values =  strs.split(&['@', ','][..]).map(|n| n.trim().parse::<i128>().unwrap()).collect::<Vec<i128>>();
            data.push(
                Hailstone::new(values[0], values[1], values[2], values[3], values[4], values[5])
            );
        });
    data
}

fn get_intersections1(hailstones: Vec<Hailstone> ) -> i128 {
    let mut total = 0;

    for (i, hs1) in hailstones.iter().enumerate() {
        for hs2 in hailstones[0..i].iter() {
            let (a1, b1, c1) = (hs1.a, hs1.b, hs1.c);
            let (a2, b2, c2) = (hs2.a, hs2.b, hs2.c);
            if a1 * b2 == b1 * a2 {
                continue;
            }
            let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
            let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);
            if 7 <= x && x <= 27
                && 7 <= y && y <= 27
                && [hs1, hs2].iter().all(|hs| (x - hs.sx) * hs.vx >= 0 && (y - hs.sy) * hs.vy >= 0)
            {
                total += 1;
            }
        }
    }

    total
}

fn get_intersections2(hailstones: Vec<Hailstone> ) -> i128 {
    let mut total = 0;

    for (i, hs1) in hailstones.iter().enumerate() {
        for hs2 in hailstones[0..i].iter() {
            let (a1, b1, c1) = (hs1.a, hs1.b, hs1.c);
            let (a2, b2, c2) = (hs2.a, hs2.b, hs2.c);
            if a1 * b2 == b1 * a2 {
                continue;
            }
            let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
            let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);
            if 200000000000000 <= x && x <= 400000000000000
                && 200000000000000 <= y && y <= 400000000000000
                && [hs1, hs2].iter().all(|hs| (x - hs.sx) * hs.vx >= 0 && (y - hs.sy) * hs.vy >= 0)
            {
                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn day24_1_test() {
        let data = format_data(Path::new("src/day24/day24_input_test.txt"));
        let result = get_intersections1(data);
        assert_eq!(result, 2);
    }

    #[test]
    fn day24_1_answer() {
        let data = format_data(Path::new("src/day24/day24_input.txt"));
        let result = get_intersections2(data);
        assert_eq!(result, 24627);
    }

    #[test]
    fn day24_2_test() {
        let data = format_data(Path::new("src/day24/day24_input_test.txt"));
        let result = get_intersections2(data);
        assert_eq!(result, 47);
    }

    #[test]
    fn day24_2_answer() {
        let data = format_data(Path::new("src/day24/day24_input.txt"));
        let result = get_intersections2(data);
        assert_eq!(result, 1);
    }
}

