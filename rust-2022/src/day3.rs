use std::collections::HashSet;

use aoc_utils::Day;
use aoc_utils::{AdventError, Answer};

pub struct Day3;

impl Day for Day3 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let total_priority = input
            .lines()
            .map(|rucksack| {
                let rucksack = rucksack.trim();

                let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

                let first_compartment: HashSet<char> = first_compartment.chars().collect();
                let second_compartment: HashSet<char> = second_compartment.chars().collect();

                let misplaced_item = first_compartment
                    .intersection(&second_compartment)
                    .next()
                    .unwrap();
                let misplaced_item_ascii = *misplaced_item as u32;

                let priority = match misplaced_item_ascii {
                    (65..=90) => misplaced_item_ascii - 38,  // ASCII A - Z
                    (97..=122) => misplaced_item_ascii - 96, // ASCII a - z
                    _ => unreachable!("Impossible item found"),
                };

                priority as u64
            })
            .sum();

        Ok(total_priority)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<&str> = input.lines().collect();
        let total_priority = lines
            .chunks(3)
            .map(|elf_group| {
                let first_elf = elf_group[0];
                let second_elf = elf_group[1];
                let third_elf = elf_group[2];

                let first_elf: HashSet<char> = first_elf.chars().collect();
                let second_elf: HashSet<char> = second_elf.chars().collect();
                let third_elf: HashSet<char> = third_elf.chars().collect();

                let common_items: HashSet<char> =
                    first_elf.intersection(&second_elf).cloned().collect();
                let mut common_items = third_elf.intersection(&common_items);

                let misplaced_item = common_items.next().unwrap();
                let misplaced_item_ascii = *misplaced_item as u32;

                let priority = match misplaced_item_ascii {
                    (65..=90) => misplaced_item_ascii - 38,  // ASCII A - Z
                    (97..=122) => misplaced_item_ascii - 96, // ASCII a - z
                    _ => unreachable!("Impossible item found"),
                };

                priority as u64
            })
            .sum();

        Ok(total_priority)
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(Day3::part_1(test_input).unwrap(), 157);
    }

    #[test]
    fn part2_works() {
        let test_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(Day3::part_2(test_input).unwrap(), 70);
    }
}
