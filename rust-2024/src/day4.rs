use aoc_utils::{input_to_trimmed_grid, test_day, AdventError, Answer, Day};

pub struct Day4;

impl Day for Day4 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let character_grid = input_to_trimmed_grid(input);

        let total = character_grid
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(column_index, c)| {
                        // Not an X, let's keep looking
                        if *c != 'X' {
                            return None;
                        }

                        let mut total_for_char = 0;

                        // Start looking for xmas
                        // 1. Check left
                        if column_index >= 3 {
                            let potential_left_m = *row.get(column_index - 1)?;
                            let potential_left_a = *row.get(column_index - 2)?;
                            let potential_left_s = *row.get(column_index - 3)?;
                            if potential_left_m == 'M'
                                && potential_left_a == 'A'
                                && potential_left_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        // 2. Check up-left
                        if column_index >= 3 && row_index >= 3 {
                            let potential_up_left_m =
                                *character_grid.get(row_index - 1)?.get(column_index - 1)?;
                            let potential_up_left_a =
                                *character_grid.get(row_index - 2)?.get(column_index - 2)?;
                            let potential_up_left_s =
                                *character_grid.get(row_index - 3)?.get(column_index - 3)?;
                            if potential_up_left_m == 'M'
                                && potential_up_left_a == 'A'
                                && potential_up_left_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        // 3. Check up
                        if row_index >= 3 {
                            let potential_up_m =
                                *character_grid.get(row_index - 1)?.get(column_index)?;
                            let potential_up_a =
                                *character_grid.get(row_index - 2)?.get(column_index)?;
                            let potential_up_s =
                                *character_grid.get(row_index - 3)?.get(column_index)?;

                            if potential_up_m == 'M'
                                && potential_up_a == 'A'
                                && potential_up_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        // 4. Check up-right
                        if column_index <= row.len() - 4 && row_index >= 3 {
                            let potential_up_right_m =
                                *character_grid.get(row_index - 1)?.get(column_index + 1)?;
                            let potential_up_right_a =
                                *character_grid.get(row_index - 2)?.get(column_index + 2)?;
                            let potential_up_right_s =
                                *character_grid.get(row_index - 3)?.get(column_index + 3)?;
                            if potential_up_right_m == 'M'
                                && potential_up_right_a == 'A'
                                && potential_up_right_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        // 5. Check right
                        if column_index <= row.len() - 4 {
                            let potential_right_m = *row.get(column_index + 1)?;
                            let potential_right_a = *row.get(column_index + 2)?;
                            let potential_right_s = *row.get(column_index + 3)?;
                            if potential_right_m == 'M'
                                && potential_right_a == 'A'
                                && potential_right_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        // 6. Check down-right
                        if column_index <= row.len() - 4 && row_index <= character_grid.len() - 4 {
                            let potential_down_right_m =
                                *character_grid.get(row_index + 1)?.get(column_index + 1)?;
                            let potential_down_right_a =
                                *character_grid.get(row_index + 2)?.get(column_index + 2)?;
                            let potential_down_right_s =
                                *character_grid.get(row_index + 3)?.get(column_index + 3)?;
                            if potential_down_right_m == 'M'
                                && potential_down_right_a == 'A'
                                && potential_down_right_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        // 7. Check down
                        if row_index <= character_grid.len() - 4 {
                            let potential_down_m =
                                *character_grid.get(row_index + 1)?.get(column_index)?;
                            let potential_down_a =
                                *character_grid.get(row_index + 2)?.get(column_index)?;
                            let potential_down_s =
                                *character_grid.get(row_index + 3)?.get(column_index)?;
                            if potential_down_m == 'M'
                                && potential_down_a == 'A'
                                && potential_down_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        // 8. Check down-left
                        if column_index >= 3 && row_index <= character_grid.len() - 4 {
                            let potential_down_left_m =
                                *character_grid.get(row_index + 1)?.get(column_index - 1)?;
                            let potential_down_left_a =
                                *character_grid.get(row_index + 2)?.get(column_index - 2)?;
                            let potential_down_left_s =
                                *character_grid.get(row_index + 3)?.get(column_index - 3)?;
                            if potential_down_left_m == 'M'
                                && potential_down_left_a == 'A'
                                && potential_down_left_s == 'S'
                            {
                                total_for_char += 1;
                            }
                        }

                        Some(total_for_char)
                    })
                    .sum::<u64>()
            })
            .sum::<u64>();

        Ok(total)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let total = 2;
        Ok(total)
    }
}

test_day!(
    Day4,
    18,
    9,
    r#"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "#
);
