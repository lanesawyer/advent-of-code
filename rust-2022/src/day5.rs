use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input.lines();

        let mut initial_stacks: Vec<&str> = lines
            .by_ref()
            .map(|line| {
                // First empty line is the end of the crates picture
                if line.is_empty() {
                    return None;
                }

                Some(line)
            })
            .take_while(|result| result.is_some())
            .map(|item| item.unwrap())
            .collect();

        let stack_positions = initial_stacks.pop().unwrap().split_whitespace().count();

        let mut stacks: Vec<Vec<String>> = (0..stack_positions).map(|_| Vec::new()).collect();
        initial_stacks.reverse();

        for crate_level in initial_stacks {
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

        let mut answer = "".to_string();

        for mut stack in stacks {
            let top_value = stack.pop().unwrap();
            answer += &top_value;
        }

        println!("  Part 1 true answer: {}", answer);

        // TODO: Answer needs to support String too
        Ok(0)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input.lines();

        let mut initial_stacks: Vec<&str> = lines
            .by_ref()
            .map(|line| {
                // First empty line is the end of the crates picture
                if line.is_empty() {
                    return None;
                }

                Some(line)
            })
            .take_while(|result| result.is_some())
            .map(|item| item.unwrap())
            .collect();

        let stack_positions = initial_stacks.pop().unwrap().split_whitespace().count();

        let mut stacks: Vec<Vec<String>> = (0..stack_positions).map(|_| Vec::new()).collect();
        initial_stacks.reverse();

        for crate_level in initial_stacks {
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

            let mut temp_stack: Vec<String> = (0..how_many)
                .into_iter()
                .map(|_| stacks[from - 1].pop().unwrap())
                .collect();

            temp_stack.reverse();

            stacks[to - 1].append(&mut temp_stack);
        }

        let mut answer = "".to_string();

        for mut stack in stacks {
            let top_value = stack.pop().unwrap();
            answer += &top_value;
        }

        println!("  Part 2 true answer: {}", answer);

        // TODO: Answer needs to support String too
        Ok(0)
    }
}

test_day!(
    Day5,
    1234, // CMZ real answer 1
    1234, // MCD real answer 2
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
