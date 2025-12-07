use aoc_utils::run_day;

mod day1;
mod day2;

use crate::day1::Day1;
use crate::day2::Day2;

fn main() {
    println!("🎄🎅🎁☃️ Advent of Code 2025 ☃️🎁🎅🎄");

    run_day!(Day1, 1);
    run_day!(Day2, 2);
}
