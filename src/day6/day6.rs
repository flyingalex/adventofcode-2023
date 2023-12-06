use std::path::Path;
use crate::utils::read_lines;

#[derive(Debug)]
struct RaceRecord {
    time: u32,
    distance: u32
}

fn beat_record(file: &Path) -> u32 {
    let mut line_nums: Vec<Vec<u32>> = vec![];
        read_lines(file)
            .unwrap()
            .for_each(|l| {
                let strs = l.unwrap();
                line_nums.push(strs.split(' ').into_iter().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>());
            });
    let mut race_records: Vec<RaceRecord> = vec![];
    for i in 0..line_nums[0].len() {
        race_records.push(RaceRecord { time: line_nums[0][i], distance: line_nums[1][i] });
    }

    println!("count: {:?}", race_records);

    let mut reach_count = vec![];
    for rr in race_records {
        let mut count = 0;
        for second in 1..=rr.time {
            if second*(rr.time - second) > rr.distance {
                count += 1;
            }
        }
        println!("count: {:?}", count);
        reach_count.push(count);
    }

    reach_count.iter().product()
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn day6_1_test() {
        let result = beat_record(Path::new("src/day6/day6_input_test.txt"));
        assert_eq!(result, 288);
    }

    #[test]
    fn day6_1_answer() {
        let result = beat_record(Path::new("src/day6/day6_input.txt"));
        assert_eq!(result, 806029445);
    }

    #[test]
    fn day6_2_test() {
        let result = beat_record(Path::new("src/day6/day6_input_test.txt"));
        assert_eq!(result, 30);
    }

    #[test]
    fn day6_2_answer() {
        let result = beat_record(Path::new("src/day6/day6_input.txt"));
        assert_eq!(result, 6284877);
    }
}
