use std::{collections::HashSet, hash::Hash, str::FromStr};

pub type Answer = u32;

pub trait Day {
    fn part_1(day: &str) -> Option<Answer>;
    fn part_2(day: &str) -> Option<Answer>;
}

pub struct Day1;

impl Day for Day1 {
    fn part_1(input: &str) -> Option<Answer> {
        let numbers = parse_input::<u32>(input);

        // Approach: Go through each number in the list
        // and search all the other numbers in the list
        // for something that adds up to 2020

        // First Attempt
        // for n in &numbers {
        //     let to_find = 2020 - n;
        //     if numbers.contains(&to_find) {
        //         return Some(to_find * n);
        //     }
        // }

        // Iterators!
        numbers.iter().find_map(|num| {
            let to_find = 2020 - num;
            match numbers.contains(&to_find) {
                true => Some(to_find * num),
                false => None,
            }
        })
    }

    fn part_2(input: &str) -> Option<Answer> {
        let numbers = parse_input::<u32>(input);

        // Approach: For each pair of numbers in the list
        // search all the other numbers in the list
        // for something that adds up to 2020
        for first in &numbers {
            for second in &numbers {
                if first + second > 2020 {
                    continue;
                }

                let to_find = 2020 - first - second;
                if numbers.contains(&to_find) {
                    return Some(to_find * first * second);
                }
            }
        }
        None
    }
}

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Option<Answer> {
        // Approach:
        // 1. Split input into lines based on new-lines,
        // 2. Pull out the lower and upper bounds, the character to check, and the password for each line
        // 3. Check each password based on the bounds and character
        // 4. Return how many were valid
        let num_valid_passwords = input
            .split('\n')
            .map(PasswordRule::parse)
            .filter(PasswordRule::is_valid_part1)
            .count();

        Some(num_valid_passwords as u32)
    }

    fn part_2(input: &str) -> Option<Answer> {
        // Approach:
        // 1. Split input into lines based on new-lines,
        // 2. Pull out the first and second positions, the character to look for in those positions, and the password for each line
        // 3. Check each position foro the characters
        // 4. Return valid if one and only one of the positions has the character
        let num_valid_passwords = input
            .split('\n')
            .map(PasswordRule::parse)
            .filter(PasswordRule::is_valid_part2)
            .count();

        Some(num_valid_passwords as u32)
    }
}

#[derive(Debug)]
struct PasswordRule {
    first_number: usize,
    second_number: usize,
    character: char,
    password: String,
}

impl PasswordRule {
    fn new(first_number: u32, second_number: u32, character: char, password: String) -> Self {
        Self {
            first_number: first_number as usize,
            second_number: second_number as usize,
            character,
            password
        }
    }

    fn parse(line: &str) -> PasswordRule {
        let line = line.trim();

        let mut chars = line.chars();
    
        let lower_check = chars.next().unwrap().to_digit(10).unwrap();
        // Could be the second digit or a -
        let lower_check =
            if let Some(lower_check_second_digit) = chars.next().unwrap().to_digit(10).or(None) {
                chars.next(); // Process the -
                lower_check * 10 + lower_check_second_digit
            } else {
                lower_check
            };
    
        let upper_check = chars.next().unwrap().to_digit(10).unwrap();
        // Could be the second digit or a space
        let upper_check =
            if let Some(upper_check_second_digit) = chars.next().unwrap().to_digit(10).or(None) {
                chars.next(); // Process the space
                upper_check * 10 + upper_check_second_digit
            } else {
                upper_check
            };
    
        let char_check = chars.next().unwrap();
        chars.next(); // : character
        chars.next(); // space character
    
        let password = chars.collect::<String>();
        PasswordRule::new(lower_check, upper_check, char_check, password)
    }

    fn is_valid_part1(rule: &PasswordRule) -> bool {
        let char_occurances = rule.password.matches(rule.character).count();

        // first is lower and second is upper bounds
        char_occurances >= rule.first_number && char_occurances <= rule.second_number
    }

    fn is_valid_part2(rule: &PasswordRule) -> bool {
        let first_position = rule.password.chars().nth(rule.first_number - 1).unwrap();
        let second_position = rule.password.chars().nth(rule.second_number - 1).unwrap();

        (first_position == rule.character && second_position != rule.character)
            || (second_position == rule.character && first_position != rule.character)
    }
}

fn parse_input<T: FromStr + Eq + Hash>(input: &str) -> HashSet<T> {
    input
        .split_whitespace()
        .filter_map(|number| number.parse::<T>().ok())
        .collect::<HashSet<T>>()
}

#[cfg(test)]
mod tests {
    use crate::{Day, Day1, Day2};

    #[test]
    fn day1_part1_works() {
        let test_input = r#"1721
            979
            366
            299
            675
            1456"#;
        assert_eq!(Day1::part_1(test_input), Some(514579));
    }

    #[test]
    fn day1_part2_works() {
        let test_input = r#"1721
            979
            366
            299
            675
            1456"#;
        assert_eq!(Day1::part_2(test_input), Some(241861950));
    }

    #[test]
    fn day2_part1_works() {
        let test_input = r#"1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc6
            11-13 d: ddddddddddd"#;
        assert_eq!(Day2::part_1(test_input), Some(3));
    }

    #[test]
    fn day2_part2_works() {
        let test_input = r#"1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
            11-13 d: ddddddddddddd"#;
        assert_eq!(Day2::part_2(test_input), Some(1));
    }
}
