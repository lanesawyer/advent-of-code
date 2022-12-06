use std::collections::HashSet;

use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day6;

impl Day for Day6 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let chars: Vec<char> = input.chars().collect();

        for index in 0..chars.len() {
            let set: HashSet<char> = (0..4).map(|num| {
                chars[index + num]
            }).collect();

            if set.len() == 4 {
                return Ok(index as u64 + 4);
            }
        }
        Ok(0)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let chars: Vec<char> = input.chars().collect();

        for index in 0..chars.len() {
            let set: HashSet<char> = (0..14).map(|num| {
                chars[index + num]
            }).collect();

            if set.len() == 14 {
                return Ok(index as u64 + 14);
            }
        }
        Ok(0)
    }
}

test_day!(
    Day6,
    5,
    19,
    // r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#, // TODO: Macro for different inputs to tests
    r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#
);
