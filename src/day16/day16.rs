use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::utils::read_lines;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_next_position(position: (u64, u64, Direction), data: &Vec<Vec<char>>) -> Vec<(u64, u64, Direction)> {
    // move
    let mirror = data[position.0 as usize][position.1 as usize];
    let directions = match position.2 {
        Direction::Up => {
            match mirror {
                '-' => vec![Direction::Right, Direction::Left],
                '|' | '.' => vec![Direction::Up],
                '/' => vec![Direction::Right],
                '\\' => vec![Direction::Left],
                _ => vec![]
            }
        }
        Direction::Down => {
            match mirror {
                '-' => vec![Direction::Right, Direction::Left],
                '|' | '.' => vec![Direction::Down],
                '/' => vec![Direction::Left],
                '\\' => vec![Direction::Right],
                _ => vec![]
            }
        }
        Direction::Left => {
            match mirror {
                '-' | '.' => vec![Direction::Left],
                '|' => vec![Direction::Up, Direction::Down],
                '/' => vec![Direction::Down],
                '\\' => vec![Direction::Up],
                _ => vec![]
            }
        }
        Direction::Right => {
            match mirror {
                '-' | '.' => vec![Direction::Right],
                '|' => vec![Direction::Up, Direction::Down],
                '/' => vec![Direction::Up],
                '\\' => vec![Direction::Down],
                _ => vec![]
            }
        }
    };
    directions.iter().map(|d| {
        let next_position = match d {
            Direction::Up => {
                if position.0 == 0 {
                    None
                } else {
                    Some((position.0 - 1, position.1))
                }
            }
            Direction::Down => {
                if position.0 == data.len() as u64 - 1 {
                    None
                } else {
                    Some((position.0 + 1, position.1))
                }
            }
            Direction::Left => {
                if position.1 == 0 {
                    None
                } else {
                    Some((position.0, position.1 - 1))
                }
            }
            Direction::Right => {
                if position.1 == data[position.0 as usize].len() as u64 - 1 {
                    None
                } else {
                    Some((position.0, position.1 + 1))
                }
            }
        };
        if let Some((x, y)) = next_position {
            Some((x, y, d.clone()))
        } else {
            None
        }
    })
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect()
}

fn format_data(file: &Path) -> Vec<Vec<char>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let s = strs.chars().collect::<Vec<char>>();
            data.push(s);
        });
    data
}

fn get_energized_tiles(data: Vec<Vec<char>>, starts: Vec<(u64, u64, Direction)>) -> u64 {
    let mut result = u64::MIN;
    for start in starts {
        let mut map: HashMap<(u64, u64, Direction), bool> = HashMap::from([
            (start.clone(), true),
        ]);
        let mut lights = vec![start];
        while !lights.is_empty() {
            let light = lights.pop().unwrap();
            let new_lights = get_next_position(light, &data);
            for l in new_lights {
                if !map.contains_key(&l) {
                    map.insert(l.clone(), true);
                    lights.push(l);
                }
            }
        }
        let mut unique_light_position = HashSet::new();
        map.iter().for_each(|(k, _)| {
            unique_light_position.insert((k.0, k.1));
        });
        result = result.max(unique_light_position.len() as u64)
    }
    result
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    #[test]
    fn day16_1_test() {
        let data = format_data(Path::new("src/day16/day16_input_test.txt"));
        let start = vec![(0, 0, Direction::Right)];
        let result = get_energized_tiles(data, start);
        assert_eq!(result, 46);
    }

    #[test]
    fn day16_1_answer() {
        let data = format_data(Path::new("src/day16/day16_input.txt"));
        let start = vec![(0, 0, Direction::Right)];
        let result = get_energized_tiles(data, start);
        assert_eq!(result, 6883);
    }

    #[test]
    fn day16_2_test() {
        let data = format_data(Path::new("src/day16/day16_input_test.txt"));
        let mut starts = vec![];
        for (row_idx, _) in data.iter().enumerate() {
            starts.push((row_idx as u64, 0, Direction::Right));
            starts.push((row_idx as u64, data.len() as u64 - 1, Direction::Left));
        }
        for (col_idx, _) in data[0].iter().enumerate() {
            starts.push((0, col_idx as u64, Direction::Down));
            starts.push((data[0].len() as u64 - 1, col_idx as u64, Direction::Up));
        }
        let result = get_energized_tiles(data, starts);
        assert_eq!(result, 51);
    }

    #[test]
    fn day16_2_answer() {
        let data = format_data(Path::new("src/day16/day16_input.txt"));
        let mut starts = vec![];
        for (row_idx, _) in data.iter().enumerate() {
            starts.push((row_idx as u64, 0, Direction::Right));
            starts.push((row_idx as u64, data.len() as u64 - 1, Direction::Left));
        }
        for (col_idx, _) in data[0].iter().enumerate() {
            starts.push((0, col_idx as u64, Direction::Down));
            starts.push((data[0].len() as u64 - 1, col_idx as u64, Direction::Up));
        }
        let result = get_energized_tiles(data, starts);
        assert_eq!(result, 90551);
    }
}
