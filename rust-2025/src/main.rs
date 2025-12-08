use aoc_utils::run_day;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;
use crate::day4::Day4;
use crate::day5::Day5;

fn main() {
    println!("🎄🎅🎁☃️ Advent of Code 2025 ☃️🎁🎅🎄");

    run_day!(Day1, 1);
    run_day!(Day2, 2);
    run_day!(Day3, 3);
    run_day!(Day4, 4);
    run_day!(Day5, 5);
}
