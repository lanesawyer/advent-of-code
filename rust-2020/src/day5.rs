use super::Answer;
use crate::Day;

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Option<Answer> {
        todo!()
    }

    fn part_2(input: &str) -> Option<Answer> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"1721
            979
            366
            299
            675
            1456"#;
        assert_eq!(Day5::part_1(test_input), Some(514579));
    }

    #[test]
    fn part2_works() {
        let test_input = r#"1721
            979
            366
            299
            675
            1456"#;
        assert_eq!(Day5::part_2(test_input), Some(241861950));
    }
}
