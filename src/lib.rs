#![feature(absolute_path)]
#![feature(let_chains)]

extern crate core;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;

mod utils;

trait Solution {
    fn part1_test(&self) -> u32;
    fn part1(&self) -> u32;
    fn part2_test(&self) -> u32;
    fn part2(&self) -> u32;
}
