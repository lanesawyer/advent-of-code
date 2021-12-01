use std::num::ParseIntError;

pub type Answer = u64;

pub trait Day {
    fn part_1(day_input: &str) -> Option<Answer>;
    fn part_2(day_input: &str) -> Option<Answer>;
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
