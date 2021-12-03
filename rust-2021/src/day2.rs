use aoc_utils::Answer;
use aoc_utils::Day;

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Option<Answer> {
        let measurements: Vec<usize> = input
            .split_whitespace()
            .map(|item| item.parse::<usize>().unwrap())
            .collect();
        let mut previous_measurement = 0;
        let mut total_increases = 0;

        for measurement in measurements {
            // Skip the first increase since we started at 0
            if measurement > previous_measurement && previous_measurement != 0 {
                total_increases += 1;
            }

            previous_measurement = measurement;
        }

        Some(total_increases)
    }

    fn part_2(input: &str) -> Option<Answer> {
        let measurements: Vec<usize> = input
            .split_whitespace()
            .map(|item| item.parse::<usize>().unwrap())
            .collect();

        let mut previous_window = 0;
        let mut total_increases = 0;

        for (index, measurement) in measurements.iter().enumerate() {
            if index + 2 < measurements.len() {
                let next = measurements[index + 1];
                let next_next = measurements[index + 2];
                let window_sum = measurement + next + next_next;

                if window_sum > previous_window && previous_window != 0 {
                    total_increases += 1;
                }

                previous_window = window_sum;
            }
        }

        Some(total_increases)
    }
}

#[cfg(test)]
mod tests {
    use super::Day2;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"199
            200
            208
            210
            200
            207
            240
            269
            260
            263"#;
        assert_eq!(Day2::part_1(test_input), Some(7));
    }

    #[test]
    fn part2_works() {
        let test_input = r#"199
            200
            208
            210
            200
            207
            240
            269
            260
            263"#;
        assert_eq!(Day2::part_2(test_input), Some(5));
    }
}
