use aoc_utils::Answer;
use aoc_utils::Day;

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Option<Answer> {
        let answer: (u64, u64) = input
            .lines()
            .fold((0, 0), |acc, line| -> (u64, u64) {
                let split_line: Vec<&str> = line.split_whitespace().collect();
                let direction = split_line.first().unwrap();
                let amount = split_line.get(1).unwrap().parse::<u64>().unwrap();

                match *direction {
                    "forward" => (acc.0 + amount, acc.1),
                    "down" => (acc.0, acc.1 + amount),
                    "up" => (acc.0, acc.1 - amount),
                    _ => (acc.0, acc.1 + 1),
                }
            });

        Some(answer.0 * answer.1)
    }

    fn part_2(input: &str) -> Option<Answer> {
        let answer: (u64, u64, u64) = input
            .lines()
            .fold((0, 0, 0), |acc, line| -> (u64, u64, u64) {
                let split_line: Vec<&str> = line.split_whitespace().collect();
                let direction = split_line.first().unwrap();
                let amount = split_line.get(1).unwrap().parse::<u64>().unwrap();

                match *direction {
                    "forward" => (acc.0 + amount, acc.1 + acc.2 * amount, acc.2),
                    "down" => (acc.0, acc.1, acc.2 + amount),
                    "up" => (acc.0, acc.1, acc.2 - amount),
                    _ => (acc.0, acc.1 + 1, acc.2),
                }
            });

        Some(answer.0 * answer.1)
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2"#;
        assert_eq!(Day2::part_1(test_input), Some(150));
    }

    #[test]
    fn part2_works() {
        let test_input = r#"forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2"#;
        assert_eq!(Day2::part_2(test_input), Some(900));
    }
}
