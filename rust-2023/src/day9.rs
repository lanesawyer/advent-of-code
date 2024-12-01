use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day9;

impl Day for Day9 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let answers: Answer = input_to_trimmed_lines(input)
            .map(|line| {
                let numbers: Vec<Answer> = line
                    .split_whitespace()
                    .filter_map(|word| word.parse::<u64>().ok())
                    .collect();

                extrapolate_value(&numbers)
            })
            .sum();
        Ok(1)
    }

    // At first I brute forced it, but it would take too long. Accidentally spoiled myself that least common multiplier
    // would work. My part two start should have a shameful asterisk next to it.
    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(2)
    }
}

fn extrapolate_value(numbers: &[Answer]) -> Answer {
    1
}

test_day!(
    Day9,
    114,
    6,
    r#"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "#
);
