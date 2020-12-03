use rust_2020::{day1::Day1, day2::Day2, Day};

fn main() {
    println!("==== Advent of Code 2020 ====");
    let day1 = read_input(1);
    match Day1::part_1(&day1) {
        Some(answer) => println!("  Part 1: {}", answer),
        None => println!("  Part 1: Not found"),
    }

    match Day1::part_2(&day1) {
        Some(answer) => println!("  Part 2: {}", answer),
        None => println!("  Part 2: Not found"),
    }

    let day2 = read_input(2);
    match Day2::part_1(&day2) {
        Some(answer) => println!("  Part 1: {}", answer),
        None => println!("  Part 1: Not found"),
    }

    match Day2::part_2(&day2) {
        Some(answer) => println!("  Part 2: {}", answer),
        None => println!("  Part 2: Not found"),
    }
}

fn read_input(day: u8) -> String {
    println!("Day {}", day);
    std::fs::read_to_string(format!("./input/day{}.txt", day)).expect(&format!(
        "Something went wrong reading the file for day {}",
        day
    ))
}
