use aoc_utils::{AdventError, Answer, Day, test_day};

pub struct Day3;

impl Day for Day3 {
    // I've been approaching this wrong. I should find the numbers first then check if it's
    // touching a symbol!
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut sum_of_valid_nums: Answer = 0;
        let line_length = input.trim().lines().next().unwrap().trim().len();

        let chars: Vec<char> = input.chars().filter(|char| !char.is_whitespace()).collect();
        // dbg!(line_length, &chars);
        chars.iter().enumerate().for_each(|(index, char)| {
            match char {
                // We don't care about periods or numbers
                char if *char == '.' || char.is_ascii_digit() || char.is_whitespace() => {}
                // We care about symbols (i.e. everything else)
                _ => {
                    sum_of_valid_nums += find_surrounding_numbers(&chars, line_length, index);
                }
            };
            // let next_num = '0';
            // let next_num_index_start = 0;
            // let next_num_index_end = 0;

            // while let Some((index, char)) = line.chars().enumerate().next() {
            //     match char {
            //         '.' => {
            //             continue;
            //         }
            //         char if char.is_ascii_digit() => {

            //         },
            //         _ => {

            //         }
            //     }
            // }

            // let (next_num_index_start, next_num): (usize, char) = line
            //     .chars()
            //     .enumerate()
            //     .skip_while(|(index, char)| !char.is_ascii_digit())
            //     .collect();

            //     .collect::<String>()
            //     .parse::<Answer>()
            //     .unwrap();
            // let numbers: Vec<char> = line
            //     .chars()
            //     .clone()
            //     .filter(|char| char.is_ascii_digit())
            //     .collect();
            // let symbols: Vec<Part> = line
            //     .chars()
            //     .enumerate()
            //     .filter_map(|(index, char)| {
            //         if !char.is_ascii_digit() && char != '.' {
            //             Some(Part::Symbol(index))
            //         } else {
            //             None
            //         }
            //     })
            //     .collect();

            // dbg!(numbers, symbols);
            // let mut process_periods = line.chars().enumerate().take_while(|(_, char)| *char != '.');
            // let something = if let Some((index, symbol_or_number)) = process_periods.next() {
            //     match symbol_or_number {
            //         symbol if symbol.is_ascii_digit() => {

            //             Some(Parts::Number(index, 1))
            //         },
            //         _ => {
            //             Some(Parts::Symbol(index))
            //         }
            //     }
            // } else {
            //     None
            // };
        });

        Ok(sum_of_valid_nums)
    }

    fn part_2(_input: &str) -> Result<Answer, AdventError> {
        Ok(2)
    }
}

fn find_surrounding_numbers(input: &[char], line_length: usize, index: usize) -> Answer {
    let mut surrounding_numbers: Answer = 0;

    // TODO: This doesn't work for symbols on the edges
    let left = input[index - 1];
    let diagonal_up_left = input[index - line_length - 1];
    let up = input[index - line_length];
    let diagonal_up_right = input[index - line_length + 1];
    let right = input[index + 1];
    let diagonal_down_right = input[index + line_length + 1];
    let down = input[index + line_length];
    let diagonal_down_left = input[index + line_length - 1];

    // dbg!(
    //     index,
    //     input[index],
    //     left,
    //     diagonal_up_left,
    //     up,
    //     diagonal_up_right,
    //     right,
    //     diagonal_down_right,
    //     down,
    //     diagonal_down_left
    // );

    // TODO: Take care of overlapping parsing that's currently happening...
    // x x x
    // x @ x
    // x x x
    if left.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index - 1);
    }
    if diagonal_up_left.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index - line_length - 1);
    }
    if up.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index - line_length);
    }
    if diagonal_up_right.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index - line_length + 1);
    }
    if right.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index + 1);
    }
    if diagonal_down_right.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index + line_length + 1);
    }
    if down.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index + line_length);
    }
    if diagonal_down_left.is_ascii_alphanumeric() {
        surrounding_numbers += build_number(input, index + line_length - 1);
    }

    surrounding_numbers
}

fn build_number(input: &[char], number_touching_symbol_index: usize) -> Answer {
    let first_index = number_touching_symbol_index - 2;
    let second_index = number_touching_symbol_index - 1;
    let middle_index = number_touching_symbol_index;
    let fourth_index = number_touching_symbol_index + 1;
    let fifth_index = number_touching_symbol_index + 2;

    let first = if input[first_index].is_ascii_alphanumeric() {
        Some(input[first_index].to_string())
    } else {
        None
    };
    let second = if input[second_index].is_ascii_alphanumeric() {
        Some(input[second_index].to_string())
    } else {
        None
    };
    let middle = if input[middle_index].is_ascii_alphanumeric() {
        Some(input[middle_index].to_string())
    } else {
        None
    };
    let fourth = if input[fourth_index].is_ascii_alphanumeric() {
        Some(input[fourth_index].to_string())
    } else {
        None
    };
    let fifth = if input[fifth_index].is_ascii_alphanumeric() {
        Some(input[fifth_index].to_string())
    } else {
        None
    };

    let number: Answer = format!(
        "{}{}{}{}{}",
        first.unwrap_or("".to_string()),
        second.unwrap_or("".to_string()),
        middle.unwrap_or("".to_string()),
        fourth.unwrap_or("".to_string()),
        fifth.unwrap_or("".to_string()),
    )
    .parse()
    .unwrap();

    // dbg!(number);

    number
}

#[derive(Debug)]
enum Part {
    // Index
    _Symbol(usize),
    // Start index, end index
    _Number(usize, usize, Answer),
}

test_day!(
    Day3,
    4361,
    0,
    r#"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "#
);
