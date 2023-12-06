use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day6;

impl Day for Day6 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input_to_trimmed_lines(input);

        let times_line = lines.next().unwrap();
        let times = parse_races(&times_line);

        let distances_line = lines.next().unwrap();
        let current_record_distances = parse_races(&distances_line);

        let answer: Answer = times
            .zip(current_record_distances)
            .map(|(time, current_record_distance)| {
                let winning_button_presses: Vec<Answer> = (0..time)
                    .filter(|&button_press| {
                        ways_to_win(time, button_press, current_record_distance)
                    })
                    .collect();

                let ways_to_win: Answer = winning_button_presses.len().try_into().unwrap();
                ways_to_win
            })
            .product();

        Ok(answer)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input_to_trimmed_lines(input);

        let times_line = lines.next().unwrap();
        let time = parse_race(&times_line);

        let distances_line = lines.next().unwrap();
        let current_record_distance = parse_race(&distances_line);

        let ways_to_win: Vec<Answer> = (0..time)
            .filter(|&button_press| ways_to_win(time, button_press, current_record_distance))
            .collect();

        Ok(ways_to_win.len().try_into().unwrap())
    }
}

fn parse_races(line: &str) -> impl Iterator<Item = Answer> + '_ {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter(|potential_numbers| !potential_numbers.is_empty())
        .filter_map(|num| num.parse::<Answer>().ok())
}

fn parse_race(line: &str) -> Answer {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .filter(|potential_numbers| !potential_numbers.is_empty())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn ways_to_win(time: Answer, button_press: Answer, current_record_distance: Answer) -> bool {
    let remaining_time = time - button_press;
    let distance_to_check = button_press * remaining_time;
    distance_to_check > current_record_distance
}

test_day!(
    Day6,
    288,
    71503,
    r#"
        Time:      7  15   30
        Distance:  9  40  200
    "#
);
