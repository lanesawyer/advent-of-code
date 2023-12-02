use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let red_cubes = 12;
        let green_cubes = 13;
        let blue_cubes = 14;

        let sum_of_valid_games: Answer = input_to_trimmed_lines(input)
            .filter_map(|line| {
                let mut label_and_game = line.split(':');

                let game_num = label_and_game
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();

                let rounds = label_and_game.next().unwrap().split(';');

                let all_rounds_valid = rounds
                    .map(|round| round.split(',').map(parse_round).collect::<Vec<_>>())
                    .all(|round| {
                        round.iter().all(|(num, color)| match (*color, num) {
                            ("red", num) if *num > red_cubes => false,
                            ("green", num) if *num > green_cubes => false,
                            ("blue", num) if *num > blue_cubes => false,
                            _ => true,
                        })
                    });

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

                let rounds = label_and_game.next().unwrap().split(';');
                let all_rounds = rounds.map(|round| round.split(',').map(parse_round));

                let all_reds = all_rounds
                    .clone()
                    .flat_map(|round| round.filter(|(_, color)| *color == "red"));
                let all_greens = all_rounds
                    .clone()
                    .flat_map(|round| round.filter(|(_, color)| *color == "green"));
                let all_blues = all_rounds
                    .clone()
                    .flat_map(|round| round.filter(|(_, color)| *color == "blue"));

                let max_red = all_reds.max_by_key(|pull| pull.0);
                let max_green = all_greens.max_by_key(|pull| pull.0);
                let max_blue = all_blues.max_by_key(|pull| pull.0);

                max_red.unwrap_or((1, "red")).0
                    * max_green.unwrap_or((1, "green")).0
                    * max_blue.unwrap_or((1, "blue")).0
            })
            .sum();

        Ok(sum_of_powers)
    }
}

fn parse_round(round: &str) -> (u64, &str) {
    let mut stuff = round.split_whitespace();
    let num = stuff.next().unwrap().parse::<u64>().unwrap();
    let color = stuff.next().unwrap();
    (num, color)
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
