use std::{num::ParseIntError, ops::{Range, RangeInclusive}};

pub type Answer = u64;

pub trait Day {
    fn part_1(day_input: &str) -> Result<Answer, AdventError>;
    fn part_2(day_input: &str) -> Result<Answer, AdventError>;
}

#[derive(Debug)]
pub enum AdventError {
    ParseInt(ParseIntError),
    IoError(std::io::Error),
    InvalidFormat(&'static str),
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

/// Reads the input from the file for the given day.
/// Requires the file to be located in `./input/day{day}.txt`.
/// Does not do any trimming or other processing.
#[deprecated(note = "Old version, now we use include_str! for faster speeds.")]
pub fn read_input(day: u8) -> Result<String, AdventError> {
    // try include_str but use a macro to get around the compile time thingy
    let input = std::fs::read_to_string(format!("./input/day{}.txt", day))?;
    Ok(input)
}

/// Turns an input string into a iterator with trimmed lines
/// Does not filter out empty lines
pub fn trimmed_input_to_lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(|line| line.trim())
}

/// Turns an input string into a iterator with trimmed lines
/// Filters out empty lines
pub fn input_to_trimmed_lines(input: &str) -> impl Iterator<Item = String> {
    trimmed_input_to_lines(input)
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
}

pub fn input_to_trimmed_grid(input: &str) -> Vec<Vec<char>> {
    input_to_trimmed_lines(input)
        .map(|line| line.trim().chars().collect())
        .filter(|line: &Vec<char>| !line.is_empty())
        .collect()
}

// Turns a string like "5-10" into a RangeInclusive<u64>
pub fn parse_range_inclusive(s: &str) -> Result<RangeInclusive<u64>, AdventError> {
    let (start_str, end_str) = s
        .split_once('-')
        .ok_or(AdventError::InvalidFormat("Missing '-' separator"))?;

    let start = start_str.parse()?;
    let end = end_str.parse()?;

    Ok(start..=end)
}

// Turns a string like "5-10" into a Range<u64>
pub fn parse_range(s: &str) -> Result<Range<u64>, AdventError> {
    let (start_str, end_str) = s
        .split_once('-')
        .ok_or(AdventError::InvalidFormat("Missing '-' separator"))?;

    let start = start_str.parse()?;
    let end = end_str.parse()?;

    Ok(start..end)
}


#[macro_export]
macro_rules! run_day {
    ($day:ident, $day_num:expr) => {{
        use aoc_utils::Day;
        use std::time::Instant;

        let input = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/day",
            $day_num,
            ".txt"
        ));

        let part_one_start = Instant::now();

        println!("Day {}", $day_num);
        match $day::part_1(&input) {
            Ok(answer) => {
                let elapsed = part_one_start.elapsed();
                println!("  Part 1: {} ({:?})", answer, elapsed);
            }
            Err(error) => println!("  Part 1 error: {:#?}", error),
        }

        let part_two_start = Instant::now();

        match $day::part_2(&input) {
            Ok(answer) => {
                let elapsed = part_two_start.elapsed();
                println!("  Part 2: {} ({:?})", answer, elapsed);
            }
            Err(error) => println!("  Part 2 error: {:#?}", error),
        }
    }};
}

#[macro_export]
macro_rules! test_day {
    // Single test input for both parts
    ($day:ident, $answer1:expr, $answer2:expr, $test_input:expr$(,)?) => {
        #[cfg(test)]
        mod tests {
            use super::$day;
            use $crate::Day;

            #[test]
            fn part1_works() {
                assert_eq!($day::part_1($test_input).unwrap(), $answer1);
            }

            #[test]
            fn part2_works() {
                assert_eq!($day::part_2($test_input).unwrap(), $answer2);
            }
        }
    };
    // Two different test inputs for each part
    ($day:ident, $answer1:expr, $answer2:expr, $test_input:expr, $test_input2:expr) => {
        #[cfg(test)]
        mod tests {
            use super::$day;
            use $crate::Day;

            #[test]
            fn part1_works() {
                assert_eq!($day::part_1($test_input).unwrap(), $answer1);
            }

            #[test]
            fn part2_works() {
                assert_eq!($day::part_2($test_input2).unwrap(), $answer2);
            }
        }
    };
}
