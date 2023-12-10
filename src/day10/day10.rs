use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::utils::read_lines;

#[derive(Debug, Clone)]
struct StartPoint {
    row: usize,
    col: usize,
}

fn format_data(file: &Path) -> (Vec<Vec<char>>, StartPoint) {
    let mut data = vec![];
    let mut start_point = StartPoint { row: 0, col: 0 };
    let mut row = 0;
    let mut final_row = 0;
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            if final_row == 0 {
                row += 1;
            }
            let strs = l.unwrap();
            let mut line_data = strs.chars().collect::<Vec<char>>();
            if let Some(idx) = line_data.iter().position(|&c| c == 'S') {
                start_point.col = idx;
                final_row = row;
            }
            line_data.insert(0, '.');
            line_data.push('.');
            data.push(line_data);
        });
    data.insert(0, vec!['.'; data[0].len()]);
    data.push(vec!['.'; data[0].len()]);
    start_point.row = row;
    start_point.col += 1;
    (data, start_point)
}

/*
......
.7-F7-.
..FJ|7.
.SJLL7.
.|F--J.
.LJ.LJ.
......

......
...45..
..236..
.01.78.
.14567.
.23....
......
 */
fn init_start_points(tiles: &Vec<Vec<char>>, start_point: StartPoint) -> Vec<StartPoint> {
    let mut points = vec![];
    // only these can be start point at top, down left, right
    let possiable_start_point_chars = vec![
        vec!['|', '7', 'F'],
        vec!['|', 'L', 'J'],
        vec!['-', 'L', 'F'],
        vec!['-', '7', 'J'],
    ];
    [
        (start_point.row - 1, start_point.col),
        (start_point.row + 1, start_point.col),
        (start_point.row, start_point.col - 1),
        (start_point.row, start_point.col + 1),
    ].iter().enumerate().for_each(|(idx, (row, col))| {
        let start_point_char = tiles[*row][*col];
        if possiable_start_point_chars[idx].contains(&start_point_char) {
            points.push(StartPoint { row: *row, col: *col });
        }
    });
    points
}

fn get_around_points(tiles: &Vec<Vec<char>>, start_point: StartPoint) -> Vec<StartPoint> {
    let possible_chars = HashMap::from([
        ('|', vec!(vec!['7', 'F', '|'], vec!['J', 'L', '|'], vec![], vec![])),
        ('-', vec!(vec![], vec![], vec!['L', 'F', '-'], vec!['7', 'J', '-'])),
        ('L', vec!(vec!['|', '7', 'F'], vec![], vec![], vec!['-', 'J', '7'])),
        ('7', vec!(vec![], vec!['|', 'J', 'L'], vec!['-', 'L', 'F'], vec![])),
        ('F', vec!(vec![], vec!['|', 'L', 'J'], vec![], vec!['-', '7', 'J'])),
        ('J', vec!(vec!['F', '7', '|'], vec![], vec!['-', 'L', 'F'], vec![])),
        ('.', vec!(vec![], vec![], vec![], vec![])),
        ('S', vec!(vec![], vec![], vec![], vec![])),
    ]);
    let mut points = vec![];
    // up, down, left, right
    [
        (start_point.row - 1, start_point.col),
        (start_point.row + 1, start_point.col),
        (start_point.row, start_point.col - 1),
        (start_point.row, start_point.col + 1),
    ].iter().enumerate().for_each(|(idx, (row, col))| {
        let start_point_char = tiles[start_point.row][start_point.col];
        let new_point_char = tiles[*row][*col];
        if possible_chars[&start_point_char][idx].contains(&new_point_char) {
            points.push(StartPoint { row: *row, col: *col });
        }
    });
    points
}

fn get_steps(tiles: Vec<Vec<char>>, start_point: StartPoint) -> i64 {
    let mut count = 1;
    let mut looped = vec![vec![false; tiles[0].len()]; tiles.len()];
    looped[start_point.row][start_point.col] = true;
    let mut start_points = init_start_points(&tiles, start_point);
    loop {
        let mut new_start_ponts = vec![];
        for start_point in start_points.into_iter() {
            println!("start_point {:?}", start_point);
            if looped[start_point.row][start_point.col] {
                continue;
            } else {
                looped[start_point.row][start_point.col] = true;
                let returned_points = get_around_points(&tiles, start_point);
                returned_points.into_iter().for_each(|point| {
                    if !looped[point.row][point.col] {
                        println!("looped[{}][{}]: {:?}", point.row, point.col, looped[point.row][point.col]);
                        new_start_ponts.push(point);
                    }
                });
            }
        }
        count += 1;
        start_points = new_start_ponts;
        let unique_point_len = start_points.clone().iter().map(|p| (p.row, p.col)).collect::<HashSet<(usize, usize)>>().len();
        println!("start_points {:?}", start_points);
        if start_points.len() > unique_point_len {
            break;
        }
    }
    count
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    #[test]
    fn day10_1_test() {
        let (tiles, start_point) = format_data(Path::new("src/day10/day10_input_test.txt"));
        let result = get_steps(tiles, start_point);
        assert_eq!(result, 8);
    }

    #[test]
    fn day10_1_answer() {
        let (tiles, start_point) = format_data(Path::new("src/day10/day10_input.txt"));
        let result = get_steps(tiles, start_point);
        assert_eq!(result, 1641934234);
    }

    #[test]
    fn day10_2_test() {
        let (tiles, start_point) = format_data(Path::new("src/day10/day10_input_test.txt"));
        let result = get_steps(tiles, start_point);
        assert_eq!(result, 2);
    }

    #[test]
    fn day10_2_answer() {
        let (tiles, start_point) = format_data(Path::new("src/day10/day10_input.txt"));
        let result = get_steps(tiles, start_point);
        assert_eq!(result, -14929);
    }
}
