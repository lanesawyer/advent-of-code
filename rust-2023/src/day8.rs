use std::collections::HashMap;

use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day8;

impl Day for Day8 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let (instructions, network) = parse_instructions_and_network(input);

        let mut steps = 0;
        let mut instruction_position = 0;
        let mut network_name = "AAA".to_string();

        // Walk the network until we find zzz, looping to the beginning of the instructions if we reach the end
        while network_name != "ZZZ" {
            if instruction_position >= instructions.len() {
                instruction_position = 0;
            }

            let next_instruction = instructions[instruction_position];
            network_name = get_next_network_name(&network, &network_name, next_instruction);

            instruction_position += 1;
            steps += 1;
        }

        Ok(steps)
    }

    // Fairly certain brute-forcing this would work, but it's going to take way too long.
    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let (instructions, network) = parse_instructions_and_network(input);

        let starting_network_names: Vec<String> = network
            .keys()
            .filter(|&name| name.ends_with('A'))
            .cloned()
            .collect();

        let mut steps = 0;
        let mut instruction_position = 0;
        let mut network_names = starting_network_names.clone();

        // Walk the network until we find _all_ the next network names end in Z,
        // looping to the beginning of the instructions if we reach the end
        while !network_names.iter().all(|name| name.ends_with('Z')) {
            if instruction_position >= instructions.len() {
                instruction_position = 0;
            }

            dbg!(steps);
            let next_instruction = instructions[instruction_position];

            let next_network_names: Vec<String> = network_names.iter().map(|network_name| {
                get_next_network_name(&network, network_name, next_instruction)
            }).collect();

            network_names = next_network_names;
            instruction_position += 1;
            steps += 1;
        }

        Ok(steps)
    }
}

fn parse_instructions_and_network(lines: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut lines = input_to_trimmed_lines(lines);

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    let network: HashMap<String, (String, String)> = lines
        .map(|line| {
            let mut parts = line.split(" = ");
            let name = parts.next().unwrap();
            let value = parts.next().unwrap();

            let left: String = value.chars().skip(1).take(3).collect();
            let right: String = value.chars().skip(6).take(3).collect();

            (name.to_string(), (left, right))
        })
        .collect();

    (instructions, network)
}

fn get_next_network_name(
    network: &HashMap<String, (String, String)>,
    network_name: &String,
    next_instruction: char,
) -> String {
    let next_node = network.get(&network_name.to_string()).unwrap();

    match (next_instruction, next_node) {
        ('L', (left, _)) => left.to_string(),
        ('R', (_, right)) => right.to_string(),
        _ => {
            panic!("Unknown instruction")
        }
    }
}

test_day!(
    Day8,
    6,
    6,
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
    "#,
    r#"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "#
);
