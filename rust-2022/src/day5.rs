use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input.lines();

        let mut picture: Vec<&str> = Vec::new();
        for line in lines.by_ref() {
            // First empty line is the end of the crates picture
            if line.is_empty() {
                break;
            }

            picture.push(line);
        }

        let stack_positions = picture
            .pop()
            .unwrap()
            .split_whitespace()
            // refactor into .count()
            .map(|item| item.parse::<u64>().unwrap());

        let mut stacks: Vec<Vec<String>> = stack_positions.map(|_| Vec::new()).collect();
        picture.reverse();

        for crate_level in picture {
            let chars = crate_level.chars();

            for (index, item) in chars.enumerate() {
                if !item.is_alphabetic() {
                    continue;
                }
                stacks[index / 4].push(item.to_string());
            }
        }

        // Now that we have the initial state represented, start moving boxes!
        for line in lines {
            let mut parsed_movement = line.split_whitespace();
            parsed_movement.next();
            let how_many = parsed_movement.next().unwrap().parse::<usize>().unwrap();
            parsed_movement.next();
            let from = parsed_movement.next().unwrap().parse::<usize>().unwrap();
            parsed_movement.next();
            let to = parsed_movement.next().unwrap().parse::<usize>().unwrap();

            for _i in 0..how_many {
                let moved_item = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(moved_item);
            }
        }

        println!("{:?}", stacks);

        for mut stack in stacks {
            print!("{}", stack.pop().unwrap());
        }
        println!();
        Ok(0)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input.lines();

        let mut picture: Vec<&str> = Vec::new();
        for line in lines.by_ref() {
            // First empty line is the end of the crates picture
            if line.is_empty() {
                break;
            }

            picture.push(line);
        }

        let stack_positions = picture
            .pop()
            .unwrap()
            .split_whitespace()
            // refactor into .count()
            .map(|item| item.parse::<u64>().unwrap());

        let mut stacks: Vec<Vec<String>> = stack_positions.map(|_| Vec::new()).collect();
        picture.reverse();

        for crate_level in picture {
            let chars = crate_level.chars();

            for (index, item) in chars.enumerate() {
                if !item.is_alphabetic() {
                    continue;
                }
                stacks[index / 4].push(item.to_string());
            }
        }

        // Now that we have the initial state represented, start moving boxes!
        for line in lines {
            let mut parsed_movement = line.split_whitespace();
            parsed_movement.next();
            let how_many = parsed_movement.next().unwrap().parse::<usize>().unwrap();
            parsed_movement.next();
            let from = parsed_movement.next().unwrap().parse::<usize>().unwrap();
            parsed_movement.next();
            let to = parsed_movement.next().unwrap().parse::<usize>().unwrap();

            let mut temp_stack: Vec<String> = (0..how_many).into_iter().map(|_| {
                stacks[from - 1].pop().unwrap()
            }).collect();

            temp_stack.reverse();

            stacks[to -1].append(&mut temp_stack);
        }

        println!("{:?}", stacks);

        for mut stack in stacks {
            print!("{}", stack.pop().unwrap());
        }
        println!();
        Ok(0)
    }
}

test_day!(
    Day5,
    1234,
    1234,
    // Usually I indent to make it pretty but parsing needs spacing information
    r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#
);
