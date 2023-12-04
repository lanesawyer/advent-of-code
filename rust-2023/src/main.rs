use aoc_utils::{run_day, AdventError};

use crate::day1::Day1;
use crate::day2::Day2;
use crate::day3::Day3;
use crate::day4::Day4;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<(), AdventError> {
    println!("🎄🎅🎁☃️ Advent of Code 2023 ☃️🎁🎅🎄");

    run_day!(Day1, 1);
    run_day!(Day2, 2);
    run_day!(Day3, 3);
    run_day!(Day4, 4);

    Ok(())
}
