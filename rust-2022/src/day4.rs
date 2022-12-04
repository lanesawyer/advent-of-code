use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day4;

impl Day for Day4 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let fully_contained = input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                let mut elf_groups = line.split(',');
                let first_elf = elf_groups.next().unwrap();
                let second_elf = elf_groups.next().unwrap();

                let (first_elf_lower, first_elf_higher) = parse_bounds(first_elf).ok()?;
                let (second_elf_lower, second_elf_higher) = parse_bounds(second_elf).ok()?;

                let is_fully_contained = (first_elf_lower >= second_elf_lower
                    && first_elf_higher <= second_elf_higher)
                    || (second_elf_lower >= first_elf_lower
                        && second_elf_higher <= first_elf_higher);
                if is_fully_contained {
                    Some(1)
                } else {
                    None
                }
            })
            .count();
        Ok(fully_contained as u64)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let fully_contained = input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                let mut elf_groups = line.split(',');
                let first_elf = elf_groups.next().unwrap();
                let second_elf = elf_groups.next().unwrap();

                let (first_elf_lower, first_elf_higher) = parse_bounds(first_elf).ok()?;
                let (second_elf_lower, second_elf_higher) = parse_bounds(second_elf).ok()?;

                // There is horrific because there is absolutely a way to simplify this
                // but I want to get the answer first
                let is_fully_contained = (first_elf_lower >= second_elf_lower
                    && first_elf_higher <= second_elf_higher)
                    || (second_elf_lower >= first_elf_lower
                        && second_elf_higher <= first_elf_higher);
                let is_matching_borders =
                    first_elf_lower == second_elf_higher || first_elf_higher == second_elf_lower;
                let first_contains_second_lower =
                    first_elf_higher > second_elf_lower && first_elf_lower < second_elf_lower;
                let first_contains_second_higher =
                    first_elf_higher > second_elf_higher && first_elf_lower < second_elf_higher;
                let second_contains_first_lower =
                    second_elf_higher > first_elf_lower && second_elf_lower < first_elf_lower;
                let second_contains_first_higher =
                    second_elf_higher > first_elf_higher && second_elf_lower < first_elf_higher;

                if is_fully_contained
                    || is_matching_borders
                    || first_contains_second_lower
                    || first_contains_second_higher
                    || second_contains_first_lower
                    || second_contains_first_higher
                {
                    Some(1)
                } else {
                    None
                }
            })
            .count();
        Ok(fully_contained as u64)
    }
}

fn parse_bounds(grouping: &str) -> Result<(u64, u64), AdventError> {
    let grouping: Vec<&str> = grouping.split('-').collect();

    let lower_bound = grouping[0].parse::<u64>()?;
    let upper_bound = grouping[1].parse::<u64>()?;
    Ok((lower_bound, upper_bound))
}

test_day!(
    Day4,
    2,
    4,
    r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#
);
