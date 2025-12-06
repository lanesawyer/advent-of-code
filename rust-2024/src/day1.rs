use std::collections::HashMap;

use aoc_utils::{AdventError, Answer, Day, input_to_trimmed_lines, test_day};

pub struct Day1;

impl Day for Day1 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut left_list: Vec<u32> = vec![];
        let mut right_list: Vec<u32> = vec![];

        input_to_trimmed_lines(input).for_each(|line| {
            let mut split_items = line.split_whitespace();
            let left = split_items.next().unwrap().parse::<u32>().unwrap();
            let right = split_items.next().unwrap().parse::<u32>().unwrap();
            left_list.push(left);
            right_list.push(right);
        });

        left_list.sort();
        right_list.sort();

        let distances: u32 = left_list
            .iter()
            .enumerate()
            .map(|(index, left_item)| {
                let right_item = right_list.get(index).unwrap();

                left_item.abs_diff(*right_item)
            })
            .sum();

        Ok(distances.into())
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut left_list: Vec<u32> = vec![];
        let mut right_list_count: HashMap<u32, u32> = HashMap::new();

        input_to_trimmed_lines(input).for_each(|line| {
            let mut split_items = line.split_whitespace();
            let left = split_items.next().unwrap().parse::<u32>().unwrap();
            let right = split_items.next().unwrap().parse::<u32>().unwrap();
            left_list.push(left);

            right_list_count.insert(right, right_list_count.get(&right).unwrap_or(&0) + 1);
        });

        let similarity_score: u32 = left_list
            .iter()
            .map(|left_item| {
                let times_seen = right_list_count.get(left_item).unwrap_or(&0);

                left_item * times_seen
            })
            .sum();

        Ok(similarity_score.into())
    }
}

test_day!(
    Day1,
    11,
    31,
    r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#
);
