use aoc_utils::{run_day, AdventError};

use crate::day1::Day1;

mod day1;

fn main() -> Result<(), AdventError> {
    println!("🎄🎅🎁☃️ Advent of Code 2024 ☃️🎁🎅🎄");

    run_day!(Day1, 1);

    Ok(())
}
