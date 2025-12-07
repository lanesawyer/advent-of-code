use aoc_utils::{AdventError, Answer, Day, test_day};

pub struct Day1;

impl Day for Day1 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        Ok(1)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
         Ok(1)
    }
}

test_day!(
    Day1,
    11,
    31,
    r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#
);
