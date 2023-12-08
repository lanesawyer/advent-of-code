use std::{cmp::Ordering, collections::HashMap};

use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day7;

impl Day for Day7 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut all_hands: Vec<Hand> = input_to_trimmed_lines(input)
            .map(parse_hand_and_bid)
            .collect();

        // Sorts ascending, since we want the index to get larger as the hand gets better
        all_hands.sort();
        let answer = all_hands
            .iter()
            .enumerate()
            .fold(0, |acc: usize, (index, hand)| {
                acc + ((index + 1) * hand.bid)
            });

        Ok(answer.try_into().unwrap())
    }

    // Attempts:
    // 252630038 (too high)
    // 252430164 (too high)
    // 252526379 (too high)
    // 249754968
    // 251637333
    // 251859926
    // 251145497
    // 251170691
    // SOOO the problem is that I'm not handling the jokers correctly. Just doing the highest counted thing
    // probably isn't _always_ the best move. I dk. Need to take a break.
    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut all_hands: Vec<JokerHand> = input_to_trimmed_lines(input)
            .map(parse_joker_hand_and_bid)
            .collect();

        // Sorts ascending, since we want the index to get larger as the hand gets better
        all_hands.sort();
        let answer = all_hands
            .iter()
            .enumerate()
            .fold(0, |acc: usize, (index, hand)| {
                acc + ((index + 1) * hand.bid)
            });

        Ok(answer.try_into().unwrap())
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

        match hand.cmp(&other_hand) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let (deciding_card, other_deciding_card) = self
                    .cards
                    .chars()
                    .zip(other.cards.chars())
                    .find(|(card, other_card)| rank_card(card, other_card) != Ordering::Equal)
                    .unwrap();

                rank_card(&deciding_card, &other_deciding_card)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct JokerHand {
    cards: String,
    bid: usize,
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand = rank_hand_joker_style(&self.cards);
        let other_hand = rank_hand_joker_style(&other.cards);

        // if (self.cards.contains('J') || other.cards.contains('J')) && hand != other_hand {
        //     println!("{}: {} vs {}: {}", self.cards, hand, other.cards, other_hand);
        //     println!("----");
        // }

        match hand.cmp(&other_hand) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let (deciding_card, other_deciding_card) = self
                    .cards
                    .chars()
                    .zip(other.cards.chars())
                    .find(|(card, other_card)| {
                        rank_card_joker_style(card, other_card) != Ordering::Equal
                    })
                    .unwrap();

                rank_card_joker_style(&deciding_card, &other_deciding_card)
            }
        }
    }
}

/// Ranks from high to low:
/// Five of a kind
/// Four of a kind
/// Full house
/// Three of a kind
/// Two pair
/// One pair
/// High card
fn rank_hand(hand: &str) -> usize {
    let character_counts = hand.chars().fold(HashMap::new(), |mut char_count, ch| {
        *char_count.entry(ch).or_insert(0) += 1;
        char_count
    });

    let counts: Vec<usize> = character_counts.values().cloned().collect();

    if counts.contains(&5) {
        7
    } else if counts.contains(&4) {
        6
    } else if counts.contains(&3) && counts.contains(&2) {
        5
    } else if counts.contains(&3) {
        4
    } else if counts.contains(&2) && counts.len() == 3 {
        3
    } else if counts.contains(&2) {
        2
    } else {
        1
    }
}

/// Ranks from high to low:
/// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
fn rank_card(card: &char, other_card: &char) -> Ordering {
    match (card, other_card) {
        (card, other_card) if card == other_card => Ordering::Equal,
        ('A', _) => Ordering::Greater,
        (_, 'A') => Ordering::Less,
        ('K', _) => Ordering::Greater,
        (_, 'K') => Ordering::Less,
        ('Q', _) => Ordering::Greater,
        (_, 'Q') => Ordering::Less,
        ('J', _) => Ordering::Greater,
        (_, 'J') => Ordering::Less,
        ('T', _) => Ordering::Greater,
        (_, 'T') => Ordering::Less,
        // Once we're in the numbers, we can just directly compare
        (num, other_num) => num.cmp(other_num),
    }
}

/// Ranks from high to low:
/// Five of a kind
/// Four of a kind
/// Full house
/// Three of a kind
/// Two pair
/// One pair
/// High card
/// But Jokers are wild and make the hand the strongest it could be
fn rank_hand_joker_style(hand: &str) -> usize {
    let character_counts = hand.chars().fold(HashMap::new(), |mut char_count, ch| {
        *char_count.entry(ch).or_insert(0) += 1;
        char_count
    });

    let counts: Vec<usize> = character_counts.values().cloned().collect();

    let rank = if counts.contains(&5) {
        7
    } else if counts.contains(&4) {
        6
    } else if counts.contains(&3) && counts.contains(&2) {
        5
    } else if counts.contains(&3) {
        4
    } else if counts.contains(&2) && counts.len() == 3 {
        3
    } else if counts.contains(&2) {
        2
    } else {
        1
    };

    if rank + hand.chars().filter(|&ch| ch == 'J').count() > 7 {
        7
    } else {
        rank + hand.chars().filter(|&ch| ch == 'J').count()
    }
    // Thought that I could just "upgrade" based on how many jokers, since it'll always make a hand better
    // rank + hand.chars().filter(|&ch| ch == 'J').count()
}
// fn rank_hand_joker_style(hand: &str) -> usize {
//     let character_counts = hand.chars().fold(HashMap::new(), |mut char_count, ch| {
//         *char_count.entry(ch).or_insert(0) += 1;
//         char_count
//     });

//     let (highest_ranking_char, count) = character_counts
//         .iter()
//         .max_by(|&(key1, value1), &(key2, value2)| {
//             if value1 != value2 {
//                 return value1.cmp(value2);
//             }
//             rank_card_joker_style(key1, key2)
//         })
//         .unwrap();
//     let unjokered_hand = hand.replace('J', &highest_ranking_char.to_string());

//     let unjokered_character_counts =
//         unjokered_hand
//             .chars()
//             .fold(HashMap::new(), |mut char_count, ch| {
//                 *char_count.entry(ch).or_insert(0) += 1;
//                 char_count
//             });

//     let unjokered_counts: Vec<usize> = unjokered_character_counts.values().cloned().collect();
//     let counts: Vec<usize> = character_counts.values().cloned().collect();

//     if hand.contains('J') && counts.len() == 3 {
//         println!("{} {} {}", highest_ranking_char, hand, unjokered_hand);
//     }

//     if unjokered_counts.contains(&5) {
//         7
//     } else if unjokered_counts.contains(&4) {
//         6
//     } else if unjokered_counts.contains(&3) && unjokered_counts.contains(&2) {
//         5
//     } else if unjokered_counts.contains(&3) {
//         4
//     } else if unjokered_counts.contains(&2) && unjokered_counts.len() == 3 {
//         3
//     } else if unjokered_counts.contains(&2) {
//         2
//     } else {
//         1
//     }
// }
// fn rank_hand_joker_style(hand: &str) -> usize {
//     let character_counts = hand.chars().fold(HashMap::new(), |mut char_count, ch| {
//         *char_count.entry(ch).or_insert(0) += 1;
//         char_count
//     });

//     let counts: Vec<usize> = character_counts.values().cloned().collect();
//     let num_jokers = hand.chars().filter(|&ch| ch == 'J').count();

//     // Five of a kind
//     if counts.contains(&5) // AAAAA || JJJJJ
//         || (counts.contains(&4) && num_jokers == 1) // AAAAJ
//         || (counts.contains(&3) && num_jokers == 2) // AAAJJ
//         || (counts.contains(&2) && num_jokers == 3) // AAJJJ
//         || (counts.contains(&1) && num_jokers == 4)
//     // AJJJJ
//     {
//         7
//     }
//     // Four of a kind
//     else if counts.contains(&4) // AAAA1 || JJJJ1
//         || (counts.contains(&3) && num_jokers == 1) // AAA1J
//         || (counts.contains(&2) && num_jokers == 2) // AA1JJ
//         || (counts.contains(&1) && num_jokers == 3)
//     // A1JJJ
//     {
//         6
//     }
//     // Full house (TODO, missing stuff?)
//     else if counts.contains(&3) && counts.contains(&2) // AAA11 || 11JJJ
//         || (counts.contains(&3) && counts.len() == 3 && num_jokers == 1) // AAA1J
//         || (counts.contains(&2) && counts.len() == 3 && num_jokers == 2) // AA1JJ
//         || (counts.contains(&1) && counts.len() == 3 && num_jokers == 3)
//     {
//         5
//     }
//     // Three of a kind
//     else if counts.contains(&3) // AAA12 || 12JJJ
//         || counts.contains(&2) && num_jokers == 1 // AA12J
//         || (counts.contains(&1) && num_jokers == 2) // A12JJ
//     {
//         4
//     }
//     // Two pair
//     else if counts.contains(&2) && counts.len() == 3 // AA112
//         || (counts.contains(&2) && counts.len() == 4 && num_jokers == 1) // AA12J
//         || (counts.contains(&1) && num_jokers == 2) // A12JJ
//     {
//         3
//     }
//     // One pair
//     else if counts.contains(&2) // AA123
//         || (counts.contains(&1) && num_jokers == 1) // A123J
//     {
//         2
//     }
//     // High card
//     else {
//         1
//     }
// }

/// Ranks from high to low, with Joker being the lowest:
/// A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J
fn rank_card_joker_style(card: &char, other_card: &char) -> Ordering {
    match (card, other_card) {
        (card, other_card) if card == other_card => Ordering::Equal,
        ('A', _) => Ordering::Greater,
        (_, 'A') => Ordering::Less,
        ('K', _) => Ordering::Greater,
        (_, 'K') => Ordering::Less,
        ('Q', _) => Ordering::Greater,
        (_, 'Q') => Ordering::Less,
        ('T', _) => Ordering::Greater,
        (_, 'T') => Ordering::Less,
        ('J', _) => Ordering::Less,
        (_, 'J') => Ordering::Greater,
        // Once we're in the numbers, we can just directly compare
        (num, other_num) => num.cmp(other_num),
    }
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

fn parse_joker_hand_and_bid(line: String) -> JokerHand {
    let mut split = line.split_whitespace();
    let hand = split.next().unwrap();
    let bid = split.next().unwrap();
    JokerHand {
        cards: hand.to_string(),
        bid: bid.parse().unwrap(),
    }
}

test_day!(
    Day7,
    6440,
    // 5905, real answer, but I need to make it fail to get dbg! output
    2,
    r#"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "#
);
