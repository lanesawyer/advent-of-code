use aoc_utils::Day;
use aoc_utils::{AdventError, Answer};

pub struct Day3;

impl Day for Day3 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        Ok(0)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"A Y
            B X
            C Z"#;
        assert_eq!(Day3::part_1(test_input).unwrap(), 15);
    }

    #[test]
    fn part2_works() {
        let test_input = r#"A Y
            B X
            C Z"#;
        assert_eq!(Day3::part_2(test_input).unwrap(), 12);
    }
}
