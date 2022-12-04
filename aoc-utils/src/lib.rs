use std::num::ParseIntError;

pub type Answer = u64;

pub trait Day {
    fn part_1(day_input: &str) -> Result<Answer, AdventError>;
    fn part_2(day_input: &str) -> Result<Answer, AdventError>;
}

#[derive(Debug)]
pub enum AdventError {
    ParseInt(ParseIntError),
    IoError(std::io::Error),
}

impl From<ParseIntError> for AdventError {
    fn from(err: ParseIntError) -> Self {
        AdventError::ParseInt(err)
    }
}

impl From<std::io::Error> for AdventError {
    fn from(err: std::io::Error) -> Self {
        AdventError::IoError(err)
    }
}

pub fn read_input(day: u8) -> Result<String, AdventError> {
    let input = std::fs::read_to_string(format!("./input/day{}.txt", day))?;
    Ok(input)
}

#[macro_export]
macro_rules! run_day {
    ($day:ident, $day_num:expr) => {
        {
            use aoc_utils::{Day, read_input};
            let input = read_input($day_num)?;

            match $day::part_1(&input) {
                Ok(answer) => println!("Day {}, part 1: {}", $day_num, answer),
                Err(error) => println!("Error: {:#?}", error),
            }

            match $day::part_2(&input) {
                Ok(answer) => println!("Day {}, part 2: {}", $day_num, answer),
                Err(error) => println!("Error: {:#?}", error),
            }
        }
    };
}

#[macro_export]
macro_rules! test_day {
    ($day:ident, $answer1:expr, $answer2:expr, $test_input:expr) => {
        #[cfg(test)]
        mod tests {
            use super::$day;
            use $crate::Day;

            #[test]
            fn part1_works() {
                let test_input = $test_input;
                assert_eq!($day::part_1(test_input).unwrap(), $answer1);
            }

            #[test]
            fn part2_works() {
                let test_input = $test_input;
                assert_eq!($day::part_2(test_input).unwrap(), $answer2);
            }
        }
    };
}
