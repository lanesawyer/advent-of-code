use std::collections::HashMap;

use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day8;

impl Day for Day8 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input_to_trimmed_lines(input);

        let instructions: Vec<char> = lines.next().unwrap().chars().collect();

        let network: HashMap<String, (String, String)> = lines.map(|line| {
            let mut parts = line.split(" = ");
            let name = parts.next().unwrap();
            let value = parts.next().unwrap();

            let left: String = value.chars().skip(1).take(3).collect();
            let right: String = value.chars().skip(6).take(3).collect();

            (name.to_string(), (left.clone(), right.clone()))
        }).collect();

        let mut steps = 0;
        let mut network_name = "AAA";
        let mut instruction_position = 0;
        // Walk the network until we find zzz, looping over the instructions
        while network_name != "ZZZ" {
            if instruction_position >= instructions.len() {
                instruction_position = 0;
            }

            dbg!(steps, network_name, instruction_position);
            let next_instruction = instructions[instruction_position];
            let next_node = network.get(&network_name.to_string()).unwrap();

            match (next_instruction, next_node) {
                ('L', (left, _)) => {
                    network_name = left;
                },
                ('R', (_, right)) => {
                    network_name = right;
                },
                _ => { panic!("Unknown instruction")}
            }

            instruction_position += 1;
            steps += 1;
        }

        Ok(steps)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(2)
    }
}

test_day!(
    Day8,
    6,
    2020,
    // Part 1, first example, equals 2
    // r#"
    //     RL

    //     AAA = (BBB, CCC)
    //     BBB = (DDD, EEE)
    //     CCC = (ZZZ, GGG)
    //     DDD = (DDD, DDD)
    //     EEE = (EEE, EEE)
    //     GGG = (GGG, GGG)
    //     ZZZ = (ZZZ, ZZZ)
    // "#,
    r#"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "#
);
