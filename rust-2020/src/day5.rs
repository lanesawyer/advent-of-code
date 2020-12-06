use std::collections::HashSet;

use super::Answer;
use crate::Day;

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Option<Answer> {
        input.split_whitespace().map(calculate_seat_number).max()
    }

    fn part_2(input: &str) -> Option<Answer> {
        let seats = input
            .split_whitespace()
            .map(calculate_seat_number)
            .collect::<HashSet<u64>>();
            
        let min = *seats.iter().min()?;
        let max = *seats.iter().max()?;

        (min..=max).find(|expected| !seats.contains(expected))
    }
}

fn calculate_seat_number(line: &str) -> u64 {
    let (row, _) = &line[..7].chars().fold((0, 127), calculate_next_range);
    let (column, _) = &line[7..].chars().fold((0, 7), calculate_next_range);

    let ans: u64 = (row * 8) + column;
    ans
}

fn calculate_next_range((start, end): (u64, u64), next: char) -> (u64, u64) {
    let halfway = (start + end) / 2;
    match next {
        'F' | 'L' => (start, halfway),
        'B' | 'R' => (halfway + 1, end),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = "FBFBBFFRLR";
        assert_eq!(Day5::part_1(test_input), Some(357));
    }

    #[test]
    #[ignore]
    fn part2_works() {
        // Not sure how to make this easily since it wasn't provided
        let test_input = "";
        assert_eq!(Day5::part_2(test_input), Some(0));
    }
}
