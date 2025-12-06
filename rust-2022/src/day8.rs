use aoc_utils::{AdventError, Answer, Day, test_day};

pub struct Day8;

impl Day for Day8 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut visible_trees = 0;

        let rows: Vec<&str> = input.lines().collect();
        for (row_index, row) in rows.iter().enumerate() {
            let trees: Vec<char> = row.trim().chars().collect();
            for (column_index, tree) in trees.iter().enumerate() {
                // Trees on the first and last row and first and last column are all visible
                if row_index == 0
                    || row_index == rows.len() - 1
                    || column_index == 0
                    || column_index == trees.len() - 1
                {
                    visible_trees += 1;
                    println!("outside tree {} ({}, {})", tree, row_index, column_index);
                    continue;
                }

                // Otherwise we're in the middle and we need to look in all directions for a taller tree
                let current_tree_height = tree
                    .to_string()
                    .parse::<usize>()
                    .expect("every tree is a number");

                let mut any_left_trees_taller = false;
                let mut any_right_trees_taller = false;
                let mut any_up_trees_taller = false;
                let mut any_down_trees_taller = false;

                // look left
                for left_index in 0..column_index {
                    let next_tree = trees
                        .get(left_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    if next_tree >= current_tree_height {
                        any_left_trees_taller = true;
                        println!(
                            "L - taller tree {} ({}, {}) than {} ({}, {})",
                            next_tree,
                            left_index,
                            column_index,
                            current_tree_height,
                            row_index,
                            column_index
                        );
                    }
                }

                // look right
                for right_index in (column_index + 1)..trees.len() {
                    let next_tree = trees
                        .get(right_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    if next_tree >= current_tree_height {
                        any_right_trees_taller = true;
                        println!(
                            "R - taller tree {} ({}, {}) than {} ({}, {})",
                            next_tree,
                            right_index,
                            column_index,
                            current_tree_height,
                            row_index,
                            column_index
                        );
                    }
                }

                // look up
                for up_index in 0..row_index {
                    let next_tree = rows
                        .get(up_index)
                        .unwrap()
                        .trim()
                        .chars()
                        .collect::<Vec<char>>()
                        .get(column_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    if next_tree >= current_tree_height {
                        any_up_trees_taller = true;
                        println!(
                            "U - taller tree {} ({}, {}) than {} ({}, {})",
                            next_tree,
                            up_index,
                            row_index,
                            current_tree_height,
                            row_index,
                            column_index
                        );
                    }
                }

                // look down
                for down_index in (row_index + 1)..rows.len() {
                    let next_tree = rows
                        .get(down_index)
                        .unwrap()
                        .trim()
                        .chars()
                        .collect::<Vec<char>>()
                        .get(column_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    if next_tree >= current_tree_height {
                        any_down_trees_taller = true;
                        println!(
                            "D - taller tree {} ({}, {}) than {} ({}, {})",
                            next_tree,
                            row_index,
                            down_index,
                            current_tree_height,
                            row_index,
                            column_index
                        );
                    }
                }

                if !any_left_trees_taller
                    || !any_right_trees_taller
                    || !any_up_trees_taller
                    || !any_down_trees_taller
                {
                    visible_trees += 1;
                    println!(
                        "{} ({}, {}) is visible",
                        current_tree_height, row_index, column_index
                    );
                }
            }
        }
        Ok(visible_trees)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut best_scenic_score = 0;

        let rows: Vec<&str> = input.lines().collect();
        for (row_index, row) in rows.iter().enumerate() {
            let trees: Vec<char> = row.trim().chars().collect();
            for (column_index, tree) in trees.iter().enumerate() {
                let current_tree_height = tree
                    .to_string()
                    .parse::<usize>()
                    .expect("every tree is a number");

                let mut left_scenic_score = 0;
                let mut right_scenic_score = 0;
                let mut top_scenic_score = 0;
                let mut bottom_scenic_score = 0;

                // Skip outer column
                let left_start = if column_index == 0 { 0 } else { column_index };
                // look left
                for left_index in (0..left_start).rev() {
                    let next_tree = trees
                        .get(left_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    left_scenic_score += 1;

                    if next_tree >= current_tree_height {
                        println!(
                            "L - taller tree {} ({}, {}) than {} ({}, {}), score: {}",
                            next_tree,
                            left_index,
                            column_index,
                            current_tree_height,
                            row_index,
                            column_index,
                            left_scenic_score
                        );
                        break;
                    }
                }

                // look right
                for right_index in (column_index + 1)..trees.len() {
                    let next_tree = trees
                        .get(right_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    right_scenic_score += 1;

                    if next_tree >= current_tree_height {
                        println!(
                            "R - taller tree {} ({}, {}) than {} ({}, {}), score: {}",
                            next_tree,
                            right_index,
                            column_index,
                            current_tree_height,
                            row_index,
                            column_index,
                            right_scenic_score
                        );
                        break;
                    }
                }

                // Skip top row
                let up_start = if row_index == 0 { 0 } else { row_index };
                // look up
                for up_index in (0..up_start).rev() {
                    let next_tree = rows
                        .get(up_index)
                        .unwrap()
                        .trim()
                        .chars()
                        .collect::<Vec<char>>()
                        .get(column_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    top_scenic_score += 1;

                    if next_tree >= current_tree_height {
                        println!(
                            "U - taller tree {} ({}, {}) than {} ({}, {}), score: {}",
                            next_tree,
                            up_index,
                            row_index,
                            current_tree_height,
                            row_index,
                            column_index,
                            top_scenic_score
                        );
                        break;
                    }
                }

                // look down
                for down_index in (row_index + 1)..rows.len() {
                    let next_tree = rows
                        .get(down_index)
                        .unwrap()
                        .trim()
                        .chars()
                        .collect::<Vec<char>>()
                        .get(column_index)
                        .unwrap()
                        .to_string()
                        .parse::<usize>()
                        .expect("every tree is a number");

                    bottom_scenic_score += 1;

                    if next_tree >= current_tree_height {
                        println!(
                            "D - taller tree {} ({}, {}) than {} ({}, {}), score: {}",
                            next_tree,
                            row_index,
                            down_index,
                            current_tree_height,
                            row_index,
                            column_index,
                            bottom_scenic_score
                        );
                        break;
                    }
                }

                let scenic_score =
                    left_scenic_score * right_scenic_score * top_scenic_score * bottom_scenic_score;
                println!(
                    "{} ({}, {}) = {}",
                    current_tree_height, row_index, column_index, scenic_score
                );

                if best_scenic_score < scenic_score {
                    best_scenic_score = scenic_score;
                }
            }
        }
        Ok(best_scenic_score)
    }
}

test_day!(
    Day8,
    21,
    8,
    r#"30373
       25512
       65332
       33549
       35390"#
);
