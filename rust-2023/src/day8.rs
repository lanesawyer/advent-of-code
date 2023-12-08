use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day8;

impl Day for Day8 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {

        Ok(1)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(2)
    }
}

test_day!(
    Day8,
    2,
    2020,
    r#"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "#
);
