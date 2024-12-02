use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let total_safe = input_to_trimmed_lines(input)
            .map(is_safe_checker)
            .filter(|is_safe| *is_safe)
            .count();
        Ok(total_safe as u64)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(1)
    }
}

fn is_safe_checker(line: String) -> bool {
    let mut is_safe = true;
    let mut is_increasing = false;
    let mut is_decreasing = false;
    let mut prev_num: Option<u32> = None;

    line.split_whitespace().for_each(|num| {
        let num = num.parse::<u32>().unwrap();

        // If we're already not safe, just finish the loop so we can get on with life
        if !is_safe {
            return;
        }

        if let Some(pn) = prev_num {
            let diff = num.abs_diff(pn);
            if !(1..=3).contains(&diff) {
                println!("{} {} {}, was too big a gap", num, pn, diff);
                is_safe = false;
            }

            // This logic is broken
            if num > pn && !is_decreasing {
                println!("{} > {} and we've never been decreasing", num, pn);
                is_increasing = true;
            } else if num > pn && is_decreasing {
                is_safe = false;
            } else if num < pn && !is_increasing {
                println!("{} < {} and we've never been increasing", num, pn);
                is_decreasing = true;
            } else if num < pn && is_increasing {
                is_safe = false;
            }
        }

        prev_num = Some(num);
    });

    println!("{} -----", is_safe);
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
