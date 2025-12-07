use aoc_utils::run_day;

mod day1;
mod day2;
mod day3;

use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;

fn main() {
    println!("🎄🎅🎁☃️ Advent of Code 2025 ☃️🎁🎅🎄");

    run_day!(Day1, 1);
    // Day 2 part 2 is 6 minutes long :'(
    // run_day!(Day2, 2);
    run_day!(Day3, 3);
}
