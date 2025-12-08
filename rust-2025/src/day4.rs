use std::char;

use aoc_utils::{AdventError, Answer, Day, input_to_trimmed_grid, test_day};

pub struct Day4;

impl Day for Day4 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let grid = input_to_trimmed_grid(input);

        let mut movable_papers = 0;

        for (row_index, row) in grid.iter().enumerate() {
            for (col_index, _spot) in row.iter().enumerate() {
                if can_paper_be_accessed_by_forklift(&grid, row_index, col_index) {
                    movable_papers += 1;
                }
            }
        }
        Ok(movable_papers)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut grid = input_to_trimmed_grid(input);

        let mut movable_papers = 0;
        let mut removed_paper_this_iteration = true;

        while removed_paper_this_iteration {
            removed_paper_this_iteration = false;

            for (row_index, row) in grid.clone().iter().enumerate() {
                for (col_index, _spot) in row.iter().enumerate() {
                    if can_paper_be_accessed_by_forklift(&grid, row_index, col_index) {
                        movable_papers += 1;

                        removed_paper_this_iteration = true;
                        grid[row_index][col_index] = 'x';
                    }
                }
            }
        }
        Ok(movable_papers)
    }
}

fn can_paper_be_accessed_by_forklift(
    grid: &Vec<Vec<char>>,
    row_index: usize,
    col_index: usize,
) -> bool {
    let char = grid[row_index][col_index];

    // If we're not a paper, then obviously the forklift can't access a paper here
    if char != '@' {
        return false;
    }

    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1), // Up-Left
        (-1, 0),  // Up
        (-1, 1),  // Up-Right
        (0, 1),   // Right
        (1, 1),   // Down-Right
        (1, 0),   // Down
        (1, -1),  // Down-Left
        (0, -1),  // Left
    ];

    let mut paper_neighbors = 0;

    for (d_row, d_col) in DIRECTIONS.iter() {
        let row_index_to_check = row_index as isize + d_row;
        let col_index_to_check = col_index as isize + d_col;

        let char_to_check_y = grid.get(row_index_to_check as usize);
        let char_to_check = char_to_check_y.and_then(|row| row.get(col_index_to_check as usize));

        if let Some(&'@') = char_to_check {
            paper_neighbors += 1;
        }
    }

    paper_neighbors <= 3
}

test_day!(
    Day4,
    13,
    43,
    r#"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "#
);
