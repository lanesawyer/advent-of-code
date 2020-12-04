use super::Answer;
use crate::Day;

const TREE_CHAR: char = '#';

pub struct Day3;

impl Day for Day3 {
    fn part_1(input: &str) -> Option<Answer> {
        // Create the counting function with the correct right and down slope
        let tree_finder = make_tree_finder(3);

        // Read each row and clean up the data so entries can be processed
        // then use the tree finder and add up all trees
        let number_of_trees_hit = input
            .split('\n')
            .filter(|s| s.len() != 0)
            .enumerate()
            .filter_map(tree_finder)
            .sum();

        Some(number_of_trees_hit)
    }

    fn part_2(input: &str) -> Option<Answer> {
        // Create all the slopes we need to check
        let tree_finders = vec![
            (1, make_tree_finder(1)),
            (1, make_tree_finder(3)),
            (1, make_tree_finder(5)),
            (1, make_tree_finder(7)),
            (2, make_tree_finder(1)),
        ];

        // Loop over our slopes and for each one do the same process as step 1,
        // but skip over the iterator by the amount we move down each time
        let trees_multiplied = tree_finders
            .iter()
            .map(|(down, count_trees)| {
                input
                    .split('\n')
                    .filter(|s| !s.is_empty())
                    .step_by(*down)
                    .enumerate()
                    .filter_map(count_trees)
                    .sum::<u64>()
            })
            .product();

        Some(trees_multiplied)
    }
}

fn make_tree_finder(right_slope: usize) -> impl Fn((usize, &str)) -> Option<u64> {
    move |(index, line)| -> Option<u64> {
        // For the line, look for the next character based on
        // the row we're on times the right slope to find the rightward movement
        // but then we mod it by the line's length to get the wrapping behavior
        // so the pattern "repeats"
        match line.trim().chars().nth((index * right_slope) % line.len()) {
            Some(TREE_CHAR) => Some(1),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = "
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        ";
        // why doesn't this work
        assert_eq!(Day3::part_1(test_input), Some(7));
    }

    #[test]
    fn part2_works() {
        let test_input = "
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        ";
        // why doesn't this work
        assert_eq!(Day3::part_2(test_input), Some(336));
    }
}
