use rust_2020::{Day, Day1};

mod lib;

fn main() {
    println!("==== Advent of Code 2020 ====");
    let day1 = read_input(1);
    println!("{}", Day1::run(&day1));
}

fn read_input(day: u8) -> String {
    println!("  Day {}", day);
    std::fs::read_to_string(format!("./input/day{}.txt", day))
        .expect(&format!("Something went wrong reading the file for day {}", day))
}
