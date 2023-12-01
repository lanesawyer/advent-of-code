use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day1;

impl Day for Day1 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let lines: Answer = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|trimmed_calibration| {
                let first_component: u64 = trimmed_calibration
                    .chars()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    .into();
                let last_component: u64 = trimmed_calibration
                    .chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    .into();

                first_component * 10 + last_component
            })
            .sum::<Answer>();

        Ok(lines)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let number_words = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let lines: Answer = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|trimmed_calibration| {
                // Look through all the English number words
                // Find the first (or last) position of each of the ten words
                // Store the index where it is and the word itself
                // Filter out Nones (for when there are no English words in the input)
                // Find the min (or max) index where the English word shows up
                let first_word_index_tuple = number_words
                    .iter()
                    .map(|number_word| (trimmed_calibration.find(*number_word), *number_word))
                    .filter(|&(x, _)| x.is_some())
                    .min_by_key(|&(x, _)| x.unwrap())
                    // If we don't find any, set it to None and a random word from the array
                    .unwrap_or((None, number_words.first().unwrap()));
                let last_word_index_tuple = number_words
                    .iter()
                    .map(|number_word| (trimmed_calibration.rfind(number_word), *number_word))
                    .filter(|&(x, _)| x.is_some())
                    .max_by_key(|&(x, _)| x.unwrap())
                    // If we don't find any, set it to None and a random word from the array
                    .unwrap_or((None, number_words.first().unwrap()));

                // Find the first (or last) index in the input that is a number
                let first_number_index =
                    trimmed_calibration.chars().position(|c| c.is_ascii_digit());
                let last_number_index = trimmed_calibration
                    .chars()
                    .rev()
                    .position(|c| c.is_ascii_digit());

                let first_component = match (first_word_index_tuple, first_number_index) {
                    ((None, _), None) => panic!("Oh boi, nothing found"),
                    // If there is no English number word, simply get the number from the index
                    ((None, _), Some(number_index)) => trimmed_calibration
                        .chars()
                        .nth(number_index)
                        .unwrap()
                        .to_digit(10)
                        .unwrap(),
                    // If there is no number, get the index of the English number word
                    // which corresponds to its actual number
                    ((Some(_pos), num_word), None) => number_words
                        .iter()
                        .position(|word| *word == num_word)
                        .unwrap()
                        .try_into()
                        .unwrap(),
                    ((Some(pos), num_word), Some(number_index)) => {
                        if pos < number_index {
                            get_number_for_word(&number_words, num_word)
                        } else {
                            trimmed_calibration
                                .chars()
                                .nth(number_index)
                                .unwrap()
                                .to_digit(10)
                                .unwrap()
                        }
                    }
                };

                let second_component = match (last_word_index_tuple, last_number_index) {
                    ((None, _), None) => panic!("Oh boi, nothing found"),
                    ((None, _), Some(number_index)) => trimmed_calibration
                        .chars()
                        .rev()
                        .nth(number_index)
                        .unwrap()
                        .to_digit(10)
                        .unwrap(),
                    ((Some(_pos), num_word), None) => number_words
                        .iter()
                        .position(|word| *word == num_word)
                        .unwrap()
                        .try_into()
                        .unwrap(),
                    ((Some(pos), num_word), Some(number_index)) => {
                        if pos > trimmed_calibration.len() - 1 - number_index {
                            get_number_for_word(&number_words, num_word)
                        } else {
                            trimmed_calibration
                                .chars()
                                .rev()
                                .nth(number_index)
                                .unwrap()
                                .to_digit(10)
                                .unwrap()
                        }
                    }
                };

                (first_component * 10 + second_component) as u64
            })
            .sum::<Answer>();

        Ok(lines)
    }
}

fn get_number_for_word(number_words: &[&str], num_word: &str) -> u32 {
    number_words
        .iter()
        .position(|word| *word == num_word)
        .unwrap()
        .try_into()
        .unwrap()
}

test_day!(
    Day1,
    142,
    281,
    // r#"
    // 1abc2
    // pqr3stu8vwx
    // a1b2c3d4e5f
    // treb7uchet
    // "#
    r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "#
);
