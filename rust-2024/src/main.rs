use aoc_utils::{run_day, AdventError};

use crate::day1::Day1;
use crate::day2::Day2;

mod day1;
mod day2;

fn main() -> Result<(), AdventError> {
    println!("🎄🎅🎁☃️ Advent of Code 2024 ☃️🎁🎅🎄");

    run_day!(Day1, 1);
    run_day!(Day2, 2);

    Ok(())
}
