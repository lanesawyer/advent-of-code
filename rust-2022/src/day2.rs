use aoc_utils::{AdventError, Answer};
use aoc_utils::Day;

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<&str> = input
            .lines()
            .collect();

        let mut current_backpack = 0;
        let mut highest_backpack = 0;

        for line in lines {
            let trimmed_line = line.trim();
            if trimmed_line == "" {
                // end of elf backpack
                if current_backpack > highest_backpack {
                    highest_backpack = current_backpack;
                }

                current_backpack = 0;
            } else {
                current_backpack += trimmed_line.parse::<u64>().unwrap();
            }
        }

        Ok(highest_backpack)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<&str> = input
            .lines()
            .collect();

        let mut current_backpack = 0;
        let mut calorie_counts: Vec<u64> = vec![];

        for (index, line) in lines.iter().enumerate()  {
            let trimmed_line = line.trim();
            if trimmed_line == "" {
                // end of elf backpack
                calorie_counts.push(current_backpack);
                current_backpack = 0;
            } else {
                current_backpack += trimmed_line.parse::<u64>().unwrap();

                // If the last backpack is one of the three top ones,
                // it gets ignored by the first if statement since there
                // are no more empty lines after it, so we check here
                if index == lines.len() - 1 {
                    calorie_counts.push(current_backpack);
                }
            }
        }

        calorie_counts.sort_by(|a, b| b.cmp(a));

        Ok(calorie_counts[0] + calorie_counts[1] + calorie_counts[2])
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000"#;
        assert_eq!(Day2::part_1(test_input).unwrap(), 24000);
    }

    #[test]
    fn part2_works() {
        let test_input = r#"1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000"#;
        assert_eq!(Day2::part_2(test_input).unwrap(), 45000);
    }
}
