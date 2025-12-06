use aoc_utils::{AdventError, Answer, Day, test_day};

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<&str> = input.lines().collect();

        let mut my_total_score = 0;

        for line in lines {
            let mut iter = line.split_whitespace();
            let enemy_action = iter.next().unwrap();
            let my_action = iter.next().unwrap();

            let match_score = match (enemy_action, my_action) {
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6, // Win
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3, // Tie
                ("A", "Z") | ("B", "X") | ("C", "Y") => 0, // Lose
                _ => panic!("Impossible situation. This isn't rock paper scissors lizard Spock!"),
            };

            // Calculate my score for my selection
            let my_action_score = match my_action {
                "X" => 1, // Rock
                "Y" => 2, // Paper
                "Z" => 3, // Scissors
                _ => panic!("Impossible situation. This isn't rock paper scissors lizard Spock!"),
            };

            my_total_score += match_score + my_action_score;
        }

        Ok(my_total_score)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let lines: Vec<&str> = input.lines().collect();

        let mut my_total_score = 0;

        for line in lines {
            let mut iter = line.split_whitespace();
            let enemy_action = iter.next().unwrap();
            let needed_result = iter.next().unwrap();

            // A is rock
            // B is paper
            // C is scissors

            // X is lose
            // Y is draw
            // Z is win

            let my_action = match (enemy_action, needed_result) {
                ("A", "Y") | ("B", "X") | ("C", "Z") => "A",
                ("A", "Z") | ("B", "Y") | ("C", "X") => "B",
                ("A", "X") | ("B", "Z") | ("C", "Y") => "C",
                _ => panic!("Impossible situation. This isn't rock paper scissors lizard Spock!"),
            };

            let match_score = match (enemy_action, my_action) {
                ("A", "B") | ("B", "C") | ("C", "A") => 6, // Win
                ("A", "A") | ("B", "B") | ("C", "C") => 3, // Tie
                ("A", "C") | ("B", "A") | ("C", "B") => 0, // Lose
                _ => panic!("Impossible situation. This isn't rock paper scissors lizard Spock!"),
            };

            // Calculate my score for my selection
            let my_action_score = match my_action {
                "A" => 1, // Rock
                "B" => 2, // Paper
                "C" => 3, // Scissors
                _ => panic!("Impossible situation. This isn't rock paper scissors lizard Spock!"),
            };

            my_total_score += match_score + my_action_score;
        }

        Ok(my_total_score)
    }
}

test_day!(
    Day2,
    15,
    12,
    r#"A Y
        B X
        C Z"#
);
