use aoc_utils::{read_input, run_day, AdventError, Day};

use crate::{day1::Day1, day2::Day2, day3::Day3, day4::Day4};

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<(), AdventError> {
    println!("==== Advent of Code 2022 ====");

    run_day!(Day1, 1);
    run_day!(Day2, 2);
    run_day!(Day3, 3);
    run_day!(Day4, 4);

    Ok(())
}
