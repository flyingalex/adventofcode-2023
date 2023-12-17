use std::collections::{HashSet};
use std::path::Path;
use std::vec;
use crate::utils::read_lines;

/*
Dijkstra
https://www.youtube.com/watch?v=JLARzu7coEs
https://www.youtube.com/watch?v=2pDSooPLLkI&t=506s
*/

fn format_data(file: &Path) -> Vec<Vec<i32>> {
    let mut data = vec![];
    read_lines(file)
        .unwrap()
        .for_each(|l| {
            let strs = l.unwrap();
            let s = strs.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
            data.push(s);
        });
    data
}

fn get_least_heat_loss(data: &mut Vec<Vec<i32>>) -> i32 {
    let mut least_heat_loss = i32::MAX;
    let max_row_idx = (data.len() - 1) as i32;
    let max_col_idx = (data[0].len() - 1) as i32;
    let mut visited: HashSet<(i32, i32, i32, i32, i32)> = HashSet::new();

    // queue: (row, col, direction_row, direction_col direction_times, heat_loss)
    let mut queue: Vec<(i32, i32, i32, i32, i32, i32)> = vec![(0, 0, 0, 0, 0, 0)];
    while !queue.is_empty() {
        // pop the minimun heat loss item
        let mut min_i = 0;
        queue.iter().enumerate().for_each(|(i, item)| {
            if item.5 < queue[min_i].5 {
                min_i = i;
            }
        });
        let (r, c, dr, dc, n, hl) = queue.remove(min_i);
        if r == max_row_idx && c == max_col_idx {
            least_heat_loss = hl;
            break;
        }

        // check duplicate
        if visited.contains(&(r, c, dr, dc, n)) {
            continue;
        }

        visited.insert((r, c, dr, dc, n));

        // move in the current direction
        if n < 3 && (dr, dc) != (0, 0) {
            let nr = r + dr;
            let nc = c + dc;
            if 0 <= nr && nr <= max_row_idx && 0 <= nc && nc <= max_col_idx {
                queue.push((nr, nc, dr, dc, n + 1, hl + data[nr as usize][nc as usize]));
            }
        }

        // move in another direction and not go back
        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
        for &(ndr, ndc) in directions.iter() {
            if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (-dr, -dc) {
                let nr = r + ndr;
                let nc = c + ndc;
                if 0 <= nr && nr <= max_row_idx && 0 <= nc && nc <= max_col_idx {
                    queue.push((nr, nc, ndr, ndc, 1, hl + data[nr as usize][nc as usize]));
                }
            }
        }
    }
    least_heat_loss
}

fn get_least_heat_loss2(data: &mut Vec<Vec<i32>>) -> i32 {
    let mut least_heat_loss = i32::MAX;
    let max_row_idx = (data.len() - 1) as i32;
    let max_col_idx = (data[0].len() - 1) as i32;
    let mut visited: HashSet<(i32, i32, i32, i32, i32)> = HashSet::new();

    // queue: (row, col, direction_row, direction_col direction_times, heat_loss)
    let mut queue: Vec<(i32, i32, i32, i32, i32, i32)> = vec![(0, 0, 0, 0, 0, 0)];
    while !queue.is_empty() {
        // pop the minimun heat loss item
        let mut min_i = 0;
        queue.iter().enumerate().for_each(|(i, item)| {
            if item.5 < queue[min_i].5 {
                min_i = i;
            }
        });
        let (r, c, dr, dc, n, hl) = queue.remove(min_i);
        if r == max_row_idx && c == max_col_idx && n >= 4 {
            least_heat_loss = hl;
            break;
        }
        if visited.contains(&(r, c, dr, dc, n)) {
            continue;
        }
        visited.insert((r, c, dr, dc, n));

        if n < 10 && (dr, dc) != (0, 0) {
            let nr = r + dr;
            let nc = c + dc;
            if 0 <= nr && nr <= max_row_idx && 0 <= nc && nc <= max_col_idx {
                queue.push((nr, nc, dr, dc, n + 1, hl + data[nr as usize][nc as usize]));
            }
        }

        if n >= 4 || (dr, dc) == (0, 0) {
            let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
            for &(ndr, ndc) in directions.iter() {
                if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (-dr, -dc) {
                    let nr = r + ndr;
                    let nc = c + ndc;
                    if 0 <= nr && nr <= max_row_idx && 0 <= nc && nc <= max_col_idx {
                        queue.push((nr, nc, ndr, ndc, 1, hl + data[nr as usize][nc as usize]));
                    }
                }
            }
        }
    }
    least_heat_loss
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn day17_1_test() {
        let mut data = format_data(Path::new("src/day17/day17_input_test.txt"));
        let result = get_least_heat_loss(&mut data);
        assert_eq!(result, 102);
    }

    #[test]
    fn day17_1_answer() {
        let mut data = format_data(Path::new("src/day17/day17_input.txt"));
        let result = get_least_heat_loss(&mut data);
        assert_eq!(result, 1155);
    }

    #[test]
    fn day17_2_test() {
        let mut data = format_data(Path::new("src/day17/day17_input_test.txt"));
        let result = get_least_heat_loss2(&mut data);
        assert_eq!(result, 94);
    }

    #[test]
    fn day17_2_answer() {
        let mut data = format_data(Path::new("src/day17/day17_input.txt"));
        let result = get_least_heat_loss2(&mut data);
        assert_eq!(result, 1283);
    }
}
