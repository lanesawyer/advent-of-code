use rust_2020::{day1::Day1, day2::Day2, day3::Day3, AdventError, Day};

fn main() -> Result<(), AdventError> {
    println!("==== Advent of Code 2020 ====");

    let day1 = read_input(1)?;
    match Day1::part_1(&day1) {
        Some(answer) => println!("  Part 1: {}", answer),
        None => println!("  Part 1: Not found"),
    }
    match Day1::part_2(&day1) {
        Some(answer) => println!("  Part 2: {}", answer),
        None => println!("  Part 2: Not found"),
    }

    let day2 = read_input(2)?;
    match Day2::part_1(&day2) {
        Some(answer) => println!("  Part 1: {}", answer),
        None => println!("  Part 1: Not found"),
    }
    match Day2::part_2(&day2) {
        Some(answer) => println!("  Part 2: {}", answer),
        None => println!("  Part 2: Not found"),
    }

    let day3 = read_input(3)?;
    match Day3::part_1(&day3) {
        Some(answer) => println!("  Part 1: {}", answer),
        None => println!("  Part 1: Not found"),
    }
    match Day3::part_2(&day3) {
        Some(answer) => println!("  Part 2: {}", answer),
        None => println!("  Part 2: Not found"),
    }

    Ok(())
}

fn read_input(day: u8) -> Result<String, AdventError> {
    println!("Day {}", day);
    let input = std::fs::read_to_string(format!("./input/day{}.txt", day))?;
    Ok(input)
}
