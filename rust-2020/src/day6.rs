use std::{collections::HashSet};

use super::Answer;
use crate::Day;

pub struct Day6;

impl Day for Day6 {
    fn part_1(input: &str) -> Option<Answer> {
        let num_questions_answered = input
            .split("\n\n")
            .map(|group| { 
                group.chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<HashSet<char>>()
                    .len()
            }).sum::<usize>();

        Some(num_questions_answered as u64)
    }

    fn part_2(input: &str) -> Option<Answer> {
        let something = input
            .split("\n\n")
            .map(|group| { 
                let chars_per_person = group.split("\n")
                    .map(|person| person.chars().collect::<HashSet<char>>());
                    //.collect::<HashSet<char>>()
                    
                    //.len();
                // add up all the characters seen between all people
                // return the ones that have the number of people in the group
                //chars_per_person
                1
            }).sum::<usize>();

        Some(something as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::Day6;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b"#;
        assert_eq!(Day6::part_1(test_input), Some(11));
    }

    #[test]
    fn part2_works() {
        // Not sure how to make this easily since it wasn't provided
        let test_input = "";
        assert_eq!(Day6::part_2(test_input), Some(0));
    }
}
