use std::collections::HashMap;

use aoc_utils::{AdventError, Answer, Day, input_to_trimmed_lines, test_day};

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

    // At first I brute forced it, but it would take too long. Accidentally spoiled myself that least common multiplier
    // would work. My part two start should have a shameful asterisk next to it.
    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let (instructions, network) = parse_instructions_and_network(input);

        let starting_network_names: Vec<String> = network
            .keys()
            .filter(|&name| name.ends_with('A'))
            .cloned()
            .collect();

        let steps_where_all_sync_up: u64 = starting_network_names
            .iter()
            .map(|name| {
                let mut steps = 0;
                let mut instruction_position = 0;
                let mut network_name = name.to_string();

                // Walk the network until we find zzz, looping to the beginning of the instructions if we reach the end
                while !network_name.ends_with('Z') {
                    if instruction_position >= instructions.len() {
                        instruction_position = 0;
                    }

                    let next_instruction = instructions[instruction_position];
                    network_name = get_next_network_name(&network, &network_name, next_instruction);

                    instruction_position += 1;
                    steps += 1;
                }

                steps
            })
            // Once we have the steps for each network, we need to find the least common multiplier
            // which is the number of steps where they all sync up. Much faster than manually walking
            // the network, which would take days, weeks, or months.
            .fold(1, |acc, steps| least_common_multiplier(acc, steps as u64));

        Ok(steps_where_all_sync_up)
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

/// Recursive function to find greatest common denominator
/// Basically, keep dividing the two numbers until the remainder is 0
/// And you've found the one where they match up!
fn greatest_common_denominator(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        greatest_common_denominator(b, a % b)
    }
}

/// Function to fined the least common multiplier
/// Following the formula:
/// a * b / gcd(a, b)
/// Where gcd is #[greatest_common_denominator]
fn least_common_multiplier(a: u64, b: u64) -> u64 {
    (a * b) / greatest_common_denominator(a, b)
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
