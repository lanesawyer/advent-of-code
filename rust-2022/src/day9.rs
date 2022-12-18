use std::collections::HashSet;

use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day9;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Position { x, y }
    }
}

impl Day for Day9 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut tail_visited: HashSet<Position> = HashSet::new();
        let mut head_position = Position::new(0, 0);
        let mut tail_position = Position::new(0, 0);

        // The tail always starts at the origin
        tail_visited.insert(Position::new(0, 0));

        input.lines().for_each(|line| {
            let mut split_line = line.split_whitespace();
            let direction = split_line.next().unwrap();
            let steps = split_line.next().unwrap().parse::<i64>().unwrap();

            println!("---- direction: {}, steps: {} ----", direction, steps);

            for _ in 0..steps {
                head_position = match direction {
                    "R" => Position::new(head_position.x + 1, head_position.y),
                    "L" => Position::new(head_position.x - 1, head_position.y),
                    "U" => Position::new(head_position.x, head_position.y + 1),
                    "D" => Position::new(head_position.x, head_position.y - 1),
                    _ => unreachable!("Invalid position"),
                };

                let x_distance = head_position.x - tail_position.x;
                let y_distance = head_position.y - tail_position.y;

                // println!("x dist: {}, y dist: {}", x_distance, y_distance);

                // There's probably a smarter way to do it than all these cardinal directions
                // Also I think when it goes into the negative these are getting messed up
                // Directly right
                if x_distance > 1 && y_distance == 0 {
                    tail_position = Position::new(head_position.x - 1, tail_position.y);
                }
                // Directly left
                else if x_distance < -1 && y_distance == 0 {
                    tail_position = Position::new(head_position.x + 1, tail_position.y);
                }
                // Directly up
                else if y_distance > 1 && x_distance == 0 {
                    tail_position = Position::new(head_position.x, tail_position.y + 1);
                }
                // Directly down
                else if y_distance < -1 && x_distance == 0 {
                    tail_position = Position::new(head_position.x, tail_position.y - 1);
                }
                // Up-right, but only snap to it when the distance is two
                else if x_distance > 0 && y_distance > 1 {
                    tail_position = Position::new(head_position.x, head_position.y - 1);
                }
                // Up-left, but only snap to it when the distance is two
                else if x_distance < -1 && y_distance > 0 {
                    tail_position = Position::new(head_position.x + 1, head_position.y);
                }
                // Down-right, but only snap to it when the distance is two
                else if x_distance > 1 && y_distance < 0 {
                    tail_position = Position::new(head_position.x - 1, head_position.y);
                }
                // Down-left, but only snap to it when the distance is two
                else if x_distance < -1 && y_distance < 0 {
                    tail_position = Position::new(head_position.x + 1, head_position.y);
                }

                tail_visited.insert(tail_position);
                println!(
                    "head: ({}, {}), tail: ({}, {})",
                    head_position.x, head_position.y, tail_position.x, tail_position.y
                );
            }
        });

        let unique_tail_visited = tail_visited.len();
        Ok(unique_tail_visited as u64)
    }

    fn part_2(_input: &str) -> Result<Answer, AdventError> {
        Ok(0)
    }
}

test_day!(
    Day9,
    13,
    8,
    r#" R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2"#
);
