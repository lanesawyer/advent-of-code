use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day4;

impl Day for Day4 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let answer: Answer = input_to_trimmed_lines(input)
            .map(|line| {
                let mut numbers = line.split(':').nth(1).unwrap().split('|');
                let input: Vec<&str> = numbers.next().unwrap().split_whitespace().collect();
                let winning_numbers: Vec<&str> =
                    numbers.next().unwrap().split_whitespace().collect();

                let matches: u32 = input
                    .iter()
                    .filter(|input| winning_numbers.contains(input))
                    .count()
                    .try_into()
                    .unwrap();

                match matches {
                    m if m > 1 => 2_u64.pow(m - 1),
                    1 => 1,
                    _ => 0,
                }
            })
            .sum();
        Ok(answer)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let card_matches: Vec<usize> = input_to_trimmed_lines(input)
            .map(|line| {
                let mut numbers = line.split(':').nth(1).unwrap().split('|');
                let input: Vec<&str> = numbers.next().unwrap().split_whitespace().collect();
                let winning_numbers: Vec<&str> =
                    numbers.next().unwrap().split_whitespace().collect();

                let matches: usize = input
                    .iter()
                    .filter(|input| winning_numbers.contains(input))
                    .count();

                matches
            })
            .collect();

        let total_card_matches = card_matches
            .iter()
            .enumerate()
            .map(|(index, card_match)| {
                if index >= card_matches.len() {
                    return 0;
                }
                // FIX THIS
                let next_cards = &card_matches[index..(index + *card_match)]
                    .iter()
                    .filter(|card| card > &&0)
                    .collect::<Vec<&usize>>()
                    .len();
                // dbg!(next_cards);
                *next_cards
            })
            .sum::<usize>()
            .try_into()
            .unwrap();

        Ok(total_card_matches)
    }
}

test_day!(
    Day4,
    13,
    30,
    r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#
);
