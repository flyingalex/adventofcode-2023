use crate::Solution;
use std::collections::HashMap;

struct Day2;
struct Part1;
struct Part2;

trait Part {
    fn calculator(&self, sub_strs: Vec<&str>) -> u32;
}

impl Part for Part1 {
    fn calculator(&self, sub_strs: Vec<&str>) -> u32 {
        let cubes_in_bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let mut id = 0;
        sub_strs.iter().for_each(|&s| {
            if s.contains("Game") {
                id = s.split(' ').last().unwrap().parse::<u32>().unwrap();
            } else {
                s.split(&[' ', ','][..])
                    .filter(|&s| !s.is_empty())
                    .collect::<Vec<&str>>()
                    .chunks(2)
                    .for_each(|w| {
                        let num = w[0].parse::<u32>().unwrap();
                        if let Some(&total) = cubes_in_bag.get(w[1])
                            && total < num
                        {
                            id = 0;
                        }
                    })
            }
        });
        id
    }
}

impl Part for Part2 {
    fn calculator(&self, sub_strs: Vec<&str>) -> u32 {
        let mut cubes_in_bag: HashMap<&str, u32> =
            HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        sub_strs.iter().for_each(|&s| {
            if !s.contains("Game") {
                s.split(&[' ', ','][..])
                    .filter(|&s| !s.is_empty())
                    .collect::<Vec<&str>>()
                    .chunks(2)
                    .for_each(|w| {
                        let num = w[0].parse::<u32>().unwrap();
                        if let Some(&max) = cubes_in_bag.get(w[1])
                            && max < num
                        {
                            cubes_in_bag.insert(w[1], num);
                        }
                    })
            }
        });
        cubes_in_bag.values().product::<u32>()
    }
}

impl Day2 {
    pub fn cube_conundrum<P: Part>(&self, part: P, input: &str) -> u32 {
        input
            .lines()
            .map(|l| {
                let sub_strs = l.split(&[':', ';'][..]).collect::<Vec<&str>>();
                part.calculator(sub_strs)
            })
            .sum()
    }
}

impl Solution for Day2 {
    fn part1_test(&self) -> u32 {
        self.cube_conundrum(Part1, include_str!("day2_input_test.txt"))
    }
    fn part1(&self) -> u32 {
        self.cube_conundrum(Part1, include_str!("day2_input.txt"))
    }

    fn part2_test(&self) -> u32 {
        self.cube_conundrum(Part2, include_str!("day2_input_test.txt"))
    }

    fn part2(&self) -> u32 {
        self.cube_conundrum(Part2, include_str!("day2_input.txt"))
    }
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn day2_1_test() {
        assert_eq!(Day2.part1_test(), 8);
    }

    #[test]
    fn day2_1_answer() {
        assert_eq!(Day2.part1(), 2169);
    }

    #[test]
    fn day2_2_test() {
        assert_eq!(Day2.part2_test(), 2286);
    }

    #[test]
    fn day2_2_answer() {
        assert_eq!(Day2.part2(), 60948);
    }
}
