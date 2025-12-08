use aoc_utils::{AdventError, Answer, Day, input_to_trimmed_lines, test_day};

pub struct Day3;

impl Day for Day3 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let result = input_to_trimmed_lines(input)
            .map(|line| {
                let mut largest: char = '0';
                let mut second_largest: char = '0';

                line.chars().enumerate().for_each(|(index, c)| {
                    let found_new_largest = c > largest && index < line.len() - 1;
                    if found_new_largest {
                        largest = c;
                        second_largest = '0';
                    }
                    if c > second_largest && !found_new_largest {
                        second_largest = c;
                    }
                });

                let pair_value = format!("{}{}", largest, second_largest);
                pair_value.parse::<u64>().unwrap()
            })
            .sum();
        Ok(result)
    }

    fn part_2(_input: &str) -> Result<Answer, AdventError> {
        Ok(0)
    }
}

test_day!(
    Day3,
    357,
    3121910778619,
    r#"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "#
);
