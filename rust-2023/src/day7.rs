use std::cmp::Ordering;

use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day7;

impl Day for Day7 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut all_hands: Vec<Hand> = input_to_trimmed_lines(input).map(parse_hand_and_bid).collect();
        all_hands.sort();
        let answer = all_hands.iter().fold(1, |index, hand| index * hand.bid);
        dbg!(all_hands);
        Ok(answer.try_into().unwrap())
    }

    fn part_2(_input: &str) -> Result<Answer, AdventError> {
        Ok(2)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand = rank_hand(&self.cards);
        let other_hand = rank_hand(&other.cards);

        if hand > other_hand {
            return Ordering::Greater;
        } else if hand < other_hand {
            return Ordering::Less;
        } else {
            return Ordering::Equal;
        }
    }
}

fn rank_hand(hand: &String) -> usize {
    1
}

fn parse_hand_and_bid(line: String) -> Hand {
    let mut split = line.split_whitespace();
    let hand = split.next().unwrap();
    let bid = split.next().unwrap();
    Hand {
        cards: hand.to_string(),
        bid: bid.parse().unwrap(),
    }
}

test_day!(
    Day7,
    6440,
    2,
    r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#
);
