use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day2;

const RED_CUBES: u64 = 12;
const GREEN_CUBES: u64 = 13;
const BLUE_CUBES: u64 = 14;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

impl Day for Day2 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let sum_of_valid_games: Answer = input_to_trimmed_lines(input)
            .filter_map(|line| {
                let mut label_and_game = line.split(':');

                let game_num = label_and_game
                    .next()?
                    .split_whitespace()
                    .nth(1)?
                    .parse::<u64>()
                    .unwrap();

                let mut rounds = label_and_game.next()?.split(';');

                let all_rounds_valid =
                    rounds.all(|round| round.split(',').map(parse_pull).all(is_valid_pull));

                if all_rounds_valid {
                    Some(game_num)
                } else {
                    None
                }
            })
            .sum();

        Ok(sum_of_valid_games)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let sum_of_powers: Answer = input_to_trimmed_lines(input)
            .map(|line| {
                let mut label_and_game = line.split(':');

                let rounds = label_and_game.nth(1).unwrap().split(';');
                let all_rounds = rounds.map(|round| round.split(',').map(parse_pull));

                let all_reds = all_rounds
                    .clone()
                    .flat_map(|round| round.filter(|(_, color)| *color == RED));
                let all_greens = all_rounds
                    .clone()
                    .flat_map(|round| round.filter(|(_, color)| *color == GREEN));
                let all_blues = all_rounds
                    .clone()
                    .flat_map(|round| round.filter(|(_, color)| *color == BLUE));

                let max_red = get_max_pull(all_reds);
                let max_green = get_max_pull(all_greens);
                let max_blue = get_max_pull(all_blues);

                max_red * max_green * max_blue
            })
            .sum();

        Ok(sum_of_powers)
    }
}

fn parse_pull(pull: &str) -> (u64, &str) {
    let mut pull_parts = pull.split_whitespace();
    let num = pull_parts.next().unwrap().parse::<u64>().unwrap();
    let color = pull_parts.next().unwrap();
    (num, color)
}

fn is_valid_pull((num, color): (u64, &str)) -> bool {
    match (num, color) {
        (num, RED) => num <= RED_CUBES,
        (num, GREEN) => num <= GREEN_CUBES,
        (num, BLUE) => num <= BLUE_CUBES,
        _ => false,
    }
}

fn get_max_pull<'a>(all_pulls_of_same_color: impl Iterator<Item = (u64, &'a str)>) -> Answer {
    all_pulls_of_same_color
        .max_by_key(|pull| pull.0)
        // If there isn't a pull for the color, we use 1 because 1 * anything is itself
        .map_or(1, |pull| pull.0)
}

test_day!(
    Day2,
    8,
    2286,
    r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#
);
