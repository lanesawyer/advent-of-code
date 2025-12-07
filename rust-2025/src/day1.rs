use aoc_utils::{AdventError, Answer, Day, input_to_trimmed_lines, test_day};

const DIAL_START: i64 = 50;

pub struct Day1;

impl Day for Day1 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let (_, num_zeros) = input_to_trimmed_lines(input)
            .map(|line| {
                let (turn, number) = line.split_at(1);
                let number: i64 = number.parse().unwrap();

                (turn.to_string(), number)
            })
            .fold((DIAL_START, 0), |acc, instruction| {
                let (next_position, num_zeros) = acc;
                // Make sure we wrap around the dial
                let (turn, mut number) = instruction;
                while number > 100 {
                    number -= 100;
                }
                let new_position = match turn.as_str() {
                    "L" => {
                        let left_position = next_position - number;
                        if left_position >= 0 {
                            left_position
                        } else {
                            100 - left_position.abs()
                        }
                    }
                    "R" => {
                        let right_position = next_position + number;
                        if right_position < 100 {
                            right_position
                        } else {
                            right_position - 100
                        }
                    }
                    _ => 0,
                };

                if new_position == 0 {
                    (new_position, num_zeros + 1)
                } else if new_position >= 0 && new_position < 100 {
                    (new_position, num_zeros)
                } else {
                    panic!("Invalid position: {}", new_position);
                }
            });

        Ok(num_zeros as u64)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let (_, num_zeros) = input_to_trimmed_lines(input)
            .map(|line| {
                let (turn, number) = line.split_at(1);
                let number: i64 = number.parse().unwrap();

                (turn.to_string(), number)
            })
            .fold((DIAL_START, 0), |acc, instruction| {
                let (next_position, num_zeros) = acc;

                let mut intermediate_zero_pass = 0;

                // Make sure we wrap around the dial
                let (turn, mut number) = instruction;
                while number > 100 {
                    intermediate_zero_pass += 1;
                    number -= 100;
                }
                let new_position = match turn.as_str() {
                    "L" => {
                        let left_position = next_position - number;
                        if left_position >= 0 {
                            left_position
                        } else {
                            if next_position != 0 {
                                intermediate_zero_pass += 1;
                            }
                            100 - left_position.abs()
                        }
                    }
                    "R" => {
                        let right_position = next_position + number;
                        if right_position < 100 {
                            right_position
                        } else {
                            if right_position != 100 {
                                intermediate_zero_pass += 1;
                            }
                            right_position - 100
                        }
                    }
                    _ => 0,
                };

                if new_position == 0 {
                    (new_position, num_zeros + 1 + intermediate_zero_pass)
                } else if new_position >= 0 && new_position < 100 {
                    (new_position, num_zeros + intermediate_zero_pass)
                } else {
                    panic!("Invalid position: {}", new_position);
                }
            });

        Ok(num_zeros as u64)
    }
}

test_day!(
    Day1,
    3,
    6,
    r#"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "#
);
