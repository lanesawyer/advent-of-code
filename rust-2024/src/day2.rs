use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let total_safe = input_to_trimmed_lines(input)
            .map(|line| is_safe_checker(&line, None))
            .filter(|is_safe| *is_safe)
            .count();
        Ok(total_safe as u64)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let total_safe = input_to_trimmed_lines(input)
            .map(|line| {
                let line_passed_with_none_removed = is_safe_checker(&line, None);
                if line_passed_with_none_removed {
                    return true;
                }

                // If it didn't fail on the first pass, go through all the possible indexes.
                // If any of them pass once an item is removed, then it's safe!
                (0..line.split_whitespace().count())
                    .any(|index| is_safe_checker(&line, Some(index)))
            })
            .filter(|is_safe| *is_safe)
            .count();

        Ok(total_safe as u64)
    }
}

fn is_safe_checker(line: &str, index_to_skip: Option<usize>) -> bool {
    let mut is_safe = true;
    let mut is_increasing = false;
    let mut is_decreasing = false;
    let mut prev_num: Option<u32> = None;

    line.split_whitespace()
        .enumerate()
        .for_each(|(index, num)| {
            // We skip indexes for part two to check if failure ones would've passed if one item was removed
            if index_to_skip.is_none() || index_to_skip.unwrap() != index {
                let num = num.parse::<u32>().unwrap();

                // If we're already not safe, just finish the loop so we can get on with life
                if !is_safe {
                    return;
                }

                // Only need to do some analysis if there is a previous number to check against
                if let Some(pn) = prev_num {
                    // If we're outside the safety range, we can immediately mark as not safe and stop looking
                    let diff = num.abs_diff(pn);
                    if !(1..=3).contains(&diff) {
                        is_safe = false;
                        return;
                    }

                    //  If the number is greater than the previous number and we've never seen a decrease,
                    //  we mark it as increasing
                    if num > pn && !is_decreasing {
                        is_increasing = true;
                    }
                    // If the number is greater than the previous number and we've seen a decrease, that means
                    // they all aren't increasing so it's marked as not safe and we can stop looking
                    else if num > pn && is_decreasing {
                        is_safe = false;
                        return;
                    }
                    // Same logic as above, but for decreasing
                    else if num < pn && !is_increasing {
                        is_decreasing = true;
                    }
                    // Same logic above, but for decreasing not holding true
                    else if num < pn && is_increasing {
                        is_safe = false;
                        return;
                    }
                }

                prev_num = Some(num);
            }
        });

    is_safe
}

test_day!(
    Day2,
    2,
    4,
    r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "#
);
