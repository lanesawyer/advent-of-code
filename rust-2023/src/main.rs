use aoc_utils::{run_day, AdventError};

// use crate::day1::Day1;
// use crate::day2::Day2;
// use crate::day3::Day3;
// use crate::day4::Day4;
// use crate::day5::Day5;
// use crate::day6::Day6;
// use crate::day7::Day7;
// use crate::day8::Day8;
use crate::day9::Day9;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), AdventError> {
    println!("🎄🎅🎁☃️ Advent of Code 2023 ☃️🎁🎅🎄");

    // run_day!(Day1, 1);
    // run_day!(Day2, 2);
    // run_day!(Day3, 3);
    // run_day!(Day4, 4);
    // run_day!(Day5, 5);
    // run_day!(Day6, 6);
    // run_day!(Day7, 7);
    // run_day!(Day8, 8);
    run_day!(Day9, 9);

    Ok(())
}
