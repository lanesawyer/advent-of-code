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
        // We need to keep track of whether we should be multiplying or not, regardless of line. It's all
        // one set of multiples, just happens to be broken up by line
        let mut should_multiply = true;

        let total = input_to_trimmed_lines(input)
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();
                let total = line
                    .chars()
                    .enumerate()
                    .map(|(index, c)| {
                        // TODO: Skip characters smartly to improve performance

                        // If we find 'don't', then turn off multiplying
                        if c == 'd'
                            && chars[index + 1] == 'o'
                            && chars[index + 2] == 'n'
                            && chars[index + 3] == '\''
                            && chars[index + 4] == 't'
                            && chars[index + 5] == '('
                            && chars[index + 6] == ')'
                        {
                            should_multiply = false;
                        }

                        // If we find 'do', then turn on multiplying
                        if c == 'd'
                            && chars[index + 1] == 'o'
                            && chars[index + 2] == '('
                            && chars[index + 3] == ')'
                        {
                            should_multiply = true;
                        }

                        // If we can't be multiplying, just move on, set to zero since the sum doesn't care
                        if !should_multiply {
                            return 0;
                        }

                        // Looks like we're going to multiply! Find the numbers
                        if c == 'm'
                            && chars[index + 1] == 'u'
                            && chars[index + 2] == 'l'
                            && chars[index + 3] == '('
                        {
                            // Find the closing bracket
                            let closing_bracket_index = chars
                                .iter()
                                // Don't need to look at the first 4 characters, as we know they're mul(
                                .skip(index + 3)
                                .position(|&c| c == ')')
                                .unwrap();

                            // Make sure it's a reasonable distance, as we can't have more than mul(XXX,YYY),
                            // so if it's further than 8 (the skip already accounts for getting in the right spot)
                            // then it's not a valid mul() block
                            if closing_bracket_index > 8 {
                                return 0;
                            }
                            let first_num_string: String = chars
                                .iter()
                                .skip(index + 4)
                                .take_while(|c| c.is_ascii_digit())
                                .collect();

                            // Then make sure we have a comma, it's required!
                            if chars[index + 3 + first_num_string.len() + 1] != ',' {
                                return 0;
                            }

                            let second_num_string: String = chars
                                .iter()
                                .skip(index + 4 + first_num_string.len() + 1)
                                .take_while(|c| c.is_ascii_digit())
                                .collect();

                            let first_num = first_num_string.parse::<u64>().unwrap();
                            let second_num = second_num_string.parse::<u64>().unwrap();

                            return first_num * second_num;
                        }

                        0
                    })
                    .sum::<u64>();

                total
            })
            .sum();
        Ok(total)
    }
}

test_day!(
    Day3,
    161,
    48,
    r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "#,
    r#"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "#
);
