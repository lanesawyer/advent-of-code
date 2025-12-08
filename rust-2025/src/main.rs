use aoc_utils::run_day;

mod day1;
mod day2;
mod day3;
mod day4;

use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;
use crate::day4::Day4;

fn main() {
    println!("🎄🎅🎁☃️ Advent of Code 2025 ☃️🎁🎅🎄");

    run_day!(Day1, 1);
    // Day 2 part 2 is 6 minutes long :'(
    // run_day!(Day2, 2);
    run_day!(Day3, 3);
    // Day 3 is 18 seconds long :'(
    // To fix, need to only check the paper spots near the one that was just removed instead of whole grid
    // run_day!(Day4, 4);
}
