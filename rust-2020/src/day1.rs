use std::{collections::HashSet, hash::Hash, str::FromStr};

use super::Answer;
use crate::Day;

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

fn parse_input<T: FromStr + Eq + Hash>(input: &str) -> HashSet<T> {
    input
        .split_whitespace()
        .filter_map(|number| number.parse::<T>().ok())
        .collect::<HashSet<T>>()
}

#[cfg(test)]
mod tests {
    use super::Day1;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"1721
            979
            366
            299
            675
            1456"#;
        assert_eq!(Day1::part_1(test_input), Some(514579));
    }

    #[test]
    fn part2_works() {
        let test_input = r#"1721
            979
            366
            299
            675
            1456"#;
        assert_eq!(Day1::part_2(test_input), Some(241861950));
    }
}
