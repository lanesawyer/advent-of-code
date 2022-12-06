use std::collections::HashSet;

use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day6;

impl Day for Day6 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let chars: Vec<char> = input.chars().collect();

        for (index, _ch) in chars.iter().enumerate() {
            let first = chars[index];
            let second = chars[index + 1];
            let third = chars[index + 2];
            let fourth = chars[index + 3];

            let set = HashSet::from([first, second, third, fourth]);

            if set.len() == 4 {
                return Ok(index as u64 + 4);
            }
        }
        Ok(0)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let chars: Vec<char> = input.chars().collect();

        for (index, _ch) in chars.iter().enumerate() {
            let first = chars[index];
            let second = chars[index + 1];
            let third = chars[index + 2];
            let fourth = chars[index + 3];
            let fifth = chars[index + 4];
            let sixth = chars[index + 5];
            let seventh = chars[index + 6];
            let eighth = chars[index + 7];
            let ninth = chars[index + 8];
            let tenth = chars[index + 9];
            let eleventh = chars[index + 10];
            let twelfth = chars[index + 11];
            let thirteenth = chars[index + 12];
            let fourteenth = chars[index + 13];

            let set = HashSet::from([
                first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth, eleventh, twelfth, thirteenth, fourteenth
            ]);

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
