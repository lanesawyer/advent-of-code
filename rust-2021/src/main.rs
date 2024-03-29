use aoc_utils::{read_input, AdventError, Day};

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), AdventError> {
    println!("==== Advent of Code 2021 ====");

    let input = read_input(1)?;

    match day1::Day1::part_1(&input) {
        Ok(answer) => println!("Day 1, part 1: {}", answer),
        Err(error) => println!("Not found: {:#?}", error),
    }

    match day1::Day1::part_2(&input) {
        Ok(answer) => println!("Day 1, part 2: {}", answer),
        Err(error) => println!("Not found: {:#?}", error),
    }

    let input = read_input(2)?;

    match day2::Day2::part_1(&input) {
        Ok(answer) => println!("Day 2, part 1: {}", answer),
        Err(error) => println!("Not found: {:#?}", error),
    }

    match day2::Day2::part_2(&input) {
        Ok(answer) => println!("Day 2, part 2: {}", answer),
        Err(error) => println!("Not found: {:#?}", error),
    }

    let input = read_input(3)?;

    match day3::Day3::part_1(&input) {
        Ok(answer) => println!("Day 3, part 1: {}", answer),
        Err(error) => println!("Not found: {:#?}", error),
    }

    match day3::Day3::part_2(&input) {
        Ok(answer) => println!("Day 3, part 2: {}", answer),
        Err(error) => println!("Not found: {:#?}", error),
    }

    Ok(())
}
