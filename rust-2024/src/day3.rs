use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};
use regex::Regex;

pub struct Day3;

impl Day for Day3 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let regex = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
        let total = input_to_trimmed_lines(input)
            .map(|line| {
                regex
                    .captures_iter(&line)
                    .map(|cap| {
                        // cap[0] is the full match, use 1 to get the first capture group, 2 for the second
                        let a = cap[1].parse::<u64>().unwrap();
                        let b = cap[2].parse::<u64>().unwrap();
                        a * b
                    })
                    .sum::<u64>()
            })
            .sum();
        Ok(total)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(2)
    }
}

test_day!(
    Day3,
    161,
    4,
    r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "#
);
