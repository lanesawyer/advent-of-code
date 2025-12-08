use aoc_utils::{AdventError, Answer, Day, input_to_grid_by_whitespace, test_day};

pub struct Day6;

impl Day for Day6 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let grid = input_to_grid_by_whitespace(input);
        let num_cols = grid[0].len();

        let mut result = 0;

        for col in 0..num_cols {
            let mut col_items = Vec::<String>::new();
            for row in grid.iter() {
                col_items.push(row[col].clone());
            }

            let operation = col_items.pop().unwrap();

            if operation == "*" {
                let product = col_items.iter().fold(1, |acc, item| {
                    acc * item.parse::<u64>().expect("Invalid number")
                });

                result += product;
            } else if operation == "+" {
                let sum = col_items.iter().fold(0, |acc, item| {
                    acc + item.parse::<u64>().expect("Invalid number")
                });

                result += sum;
            } else {
                panic!("Unknown operation: {}", operation);
            }
        }

        Ok(result)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(0)
    }
}

test_day!(
    Day6,
    4277556,
    3263827,
    r#"
        123 328  51 64 
        45 64  387 23 
        6 98  215 314
        *   +   *   +  
    "#
);
