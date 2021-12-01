use aoc_utils::{read_input, AdventError, Day};

mod day1;

fn main() -> Result<(), AdventError> {
    println!("==== Advent of Code 2020 ====");

    let input = read_input(1)?;

    match day1::Day1::part_1(&input) {
        Some(answer) => println!("Day 1, part 1: {}", answer),
        None => println!("Not found"),
    }

    match day1::Day1::part_2(&input) {
        Some(answer) => println!("Day 1, part 2: {}", answer),
        None => println!("Not found"),
    }

    Ok(())
}
