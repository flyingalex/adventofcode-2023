use std::path::Path;
use crate::utils::read_lines;

struct RaceRecord {
    time: u64,
    distance: u64
}

fn beat_record(file: &Path, combined: bool) -> u64 {
    let mut line_nums: Vec<Vec<u64>> = vec![];
        read_lines(file)
            .unwrap()
            .for_each(|l| {
                let strs = l.unwrap();
                if combined {
                    line_nums.push(vec![strs.split(' ').into_iter().filter_map(|s| s.parse::<u64>().ok()).map(|s| s.to_string()).collect::<Vec<String>>().join("").parse::<u64>().unwrap()]);
                } else {
                    line_nums.push(strs.split(' ').into_iter().filter_map(|s| s.parse::<u64>().ok()).collect::<Vec<u64>>());
                }
            });
    let mut race_records: Vec<RaceRecord> = vec![];
    for i in 0..line_nums[0].len() {
        race_records.push(RaceRecord { time: line_nums[0][i], distance: line_nums[1][i] });
    }

    let mut reach_count = vec![];
    for rr in race_records {
        let mut count = 0;
        for second in 1..=rr.time {
            if second*(rr.time - second) > rr.distance {
                count += 1;
            }
        }
        reach_count.push(count);
    }

    reach_count.iter().product()
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn day6_1_test() {
        let result = beat_record(Path::new("src/day6/day6_input_test.txt"), false);
        assert_eq!(result, 288);
    }

    #[test]
    fn day6_1_answer() {
        let result = beat_record(Path::new("src/day6/day6_input.txt"), false);
        assert_eq!(result, 440000);
    }

    #[test]
    fn day6_2_test() {
        let result = beat_record(Path::new("src/day6/day6_input_test.txt"), true);
        assert_eq!(result, 71503);
    }

    #[test]
    fn day6_2_answer() {
        let result = beat_record(Path::new("src/day6/day6_input.txt"), true);
        assert_eq!(result, 26187338);
    }
}
