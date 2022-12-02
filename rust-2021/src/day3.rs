use aoc_utils::Day;
use aoc_utils::{AdventError, Answer};

pub struct Day3;

impl Day for Day3 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<&str> = input.lines().collect();
        let total_lines = lines.len();

        let initial_vec = lines
            .get(0)
            .unwrap()
            .chars()
            .filter_map(|char| char.to_digit(10))
            .collect();

        let counts = lines.iter().fold(initial_vec, |acc, line| {
            let next_counts: Vec<u32> = line
                .chars()
                .filter_map(|number| number.to_digit(10))
                .zip(acc)
                .map(|zipup| zipup.0 + zipup.1)
                .collect();
            next_counts
        });

        let gamma_string = counts.iter().fold("".to_owned(), |acc, count| {
            format!("{}{}", acc, get_most_seen(*count, total_lines))
        });

        let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
        let epsilon_mask_string = vec!["1"; gamma_string.len()].join("");
        let epsilon_mask = usize::from_str_radix(&epsilon_mask_string, 2).unwrap();

        // mask to cut off anything past the number of digits in the input
        let epsilon = !gamma & epsilon_mask;

        Ok((gamma * epsilon) as u64)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<String> = input.lines().map(|line| line.trim().to_string()).collect();

        let mut ox_gen_rating: Option<String> = None;
        let mut loop_count = 0;
        let mut filtered_list: Vec<String> = lines.clone();

        while ox_gen_rating == None {
            let digit_count = filtered_list.iter().fold(0, |acc, line| {
                acc + line.chars().nth(loop_count).unwrap().to_digit(10).unwrap()
            });

            let total_lines = filtered_list.len();

            let filter_num = get_most_seen_char(digit_count, total_lines);

            let result: Vec<String> = filtered_list
                .iter()
                .filter(|line| line.chars().nth(loop_count).unwrap() == filter_num)
                .map(|line| line.to_string())
                .collect();

            filtered_list = result.clone();

            if result.len() == 1 {
                ox_gen_rating = Some(result.get(0).unwrap().to_string())
            } else {
                loop_count += 1;
            }
        }

        let mut co2_scrub_rating: Option<String> = None;
        loop_count = 0;
        filtered_list = lines.clone();

        while co2_scrub_rating == None {
            let digit_count = filtered_list.iter().fold(0, |acc, line| {
                acc + line.chars().nth(loop_count).unwrap().to_digit(10).unwrap()
            });

            let total_lines = filtered_list.len();

            let filter_num = get_least_seen_char(total_lines as u32 - digit_count, total_lines);

            let result: Vec<String> = filtered_list
                .iter()
                .filter(|line| line.chars().nth(loop_count).unwrap() == filter_num)
                .map(|line| line.to_string())
                .collect();

            filtered_list = result.clone();

            if result.len() == 1 {
                co2_scrub_rating = Some(result.get(0).unwrap().to_string())
            } else {
                loop_count += 1;
            }
        }

        let ox_gen_rating_decimal = usize::from_str_radix(&ox_gen_rating.unwrap(), 2).unwrap();
        let co2_scrub_rating_decimal =
            usize::from_str_radix(&co2_scrub_rating.unwrap(), 2).unwrap();
        let answer = ox_gen_rating_decimal * co2_scrub_rating_decimal;

        Ok(answer as u64)
    }
}

fn get_most_seen(times_seen: u32, total_lines: usize) -> u32 {
    if times_seen > total_lines as u32 / 2 {
        1
    } else {
        0
    }
}

fn get_most_seen_char(times_seen: u32, total_lines: usize) -> char {
    if times_seen as f32 >= total_lines as f32 / 2. {
        '1'
    } else {
        '0'
    }
}

fn get_least_seen_char(times_seen: u32, total_lines: usize) -> char {
    if times_seen as f32 <= total_lines as f32 / 2. {
        '0'
    } else {
        '1'
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"#;
        assert_eq!(Day3::part_1(test_input).unwrap(), 198);
    }

    #[test]
    fn part2_works() {
        let test_input = r#"00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010"#;
        assert_eq!(Day3::part_2(test_input).unwrap(), 230);
    }
}
