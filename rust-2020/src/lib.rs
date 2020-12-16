use std::num::ParseIntError;

pub type Answer = u64;

pub trait Day {
    fn part_1(day: &str) -> Option<Answer>;
    fn part_2(day: &str) -> Option<Answer>;
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

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
