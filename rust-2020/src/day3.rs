use super::Answer;
use crate::Day;

const TREE_CHAR: char = '#';

pub struct Day3;

impl Day for Day3 {
    fn part_1(input: &str) -> Option<Answer> {
        // Create the counting function with the correct right and down slope
        let tree_finder = make_tree_finder(3, 1);

        // Read each row and clean up the data so entries can be processed
        // then use the tree finder and add up all trees
        let number_of_trees_hit = input
            .split('\n')
            .filter(|s| s.len() != 0)
            .enumerate()
            .map(tree_finder)
            .sum();

        Some(number_of_trees_hit)
    }

    fn part_2(input: &str) -> Option<Answer> {
        // Create all the slopes we need to check
        let tree_finders = vec![
            make_tree_finder(1, 1),
            make_tree_finder(3, 1),
            make_tree_finder(5, 1),
            make_tree_finder(7, 1),
            make_tree_finder(1, 2),
        ];

        // Loop over our slopes and for each one do the same process as step 1
        let trees_multiplied = tree_finders
            .iter()
            .map(|count_trees| {
                input
                    .split('\n')
                    .filter(|s| s.trim().len() != 0)
                    .enumerate()
                    .map(count_trees)
                    .sum::<u64>()
            })
            .fold(1, |acc, trees| {
                // From the printing here I think all my down_slope = 1 things work and = 2 doesn't
                // Because 286 is the right answer for at least the second in the list, so we know the
                // algorithm works to a degree, even if the unit test doesn't
                println!("{}", trees);
                acc * trees
            });

        Some(trees_multiplied)
    }
}

fn make_tree_finder(right_slope: usize, down_slope: usize) -> impl Fn((usize, &str)) -> u64 {
    move |(index, line)| -> u64 {
        // Only even attempt to add trees if we're on a line that
        // is on the down slop, which happens when the index evenly divides
        if index % down_slope == 0 {
            // For the line, look for the next character based on
            // the row we're on times the right slope to find the rightward movement
            // but then we mod it by the line's length to get the wrapping behavior
            // so the pattern "repeats"
            match line.trim().chars().nth(index * right_slope % line.len()) {
                Some(TREE_CHAR) => 1,
                _ => 0,
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day3;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = r#"..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#"#;
        // why doesn't this work
        assert_eq!(Day3::part_1(test_input), Some(7));
    }

    #[test]
    fn part2_works() {
        let test_input = r#"..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#"#;
        // not sure how many
        assert_eq!(Day3::part_2(test_input), Some(1));
    }
}
