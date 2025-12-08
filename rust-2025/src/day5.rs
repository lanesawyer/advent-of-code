use std::ops::RangeInclusive;

use aoc_utils::{
    AdventError, Answer, Day, parse_range_inclusive, test_day, trimmed_input_to_lines,
};

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut ranges = Vec::<RangeInclusive<u64>>::new();

        let mut has_processed_all_ranges = false;
        let mut unspoiled_food = 0;

        trimmed_input_to_lines(input).for_each(|line| {
            if line.is_empty() {
                has_processed_all_ranges = true;
                return;
            }

            if !has_processed_all_ranges {
                let split_range = parse_range_inclusive(line).expect("Failed to parse range");
                ranges.push(split_range);
            } else {
                let number: u64 = line.parse().expect("Failed to parse number");
                if ranges.iter().any(|range| range.contains(&number)) {
                    unspoiled_food += 1;
                }
            }
        });

        Ok(unspoiled_food)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(0)
    }
}

test_day!(
    Day5,
    3,
    14,
    r#"3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "#
);
