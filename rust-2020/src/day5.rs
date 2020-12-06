use super::Answer;
use crate::Day;

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Option<Answer> {
        // Future me, make some closure functions to reduce duplication
        let max =  input.split_whitespace()
            .map(|line| {
                let (row, _) = &line[..7].chars()
                    .fold((0, 127), |(start, end), next| {
                        let halfway = (start + end) / 2;
                        match next {
                            'F' => (start, halfway),
                            'B' => (halfway + 1, end),
                            _ => { unreachable!( )}
                        }
                    });
                let (column, _) = &line[7..].chars()
                    .fold((0, 7), |(start, end), next| {
                        let halfway = (start + end) / 2;
                        match next {
                            'L' => (start, halfway),
                            'R' => (halfway + 1, end),
                            _ => unreachable!(),
                        }
                    });

                let ans: u64 = (row * 8) + column;
                ans
            })
            .max().unwrap();
        Some(max)
    }

    fn part_2(input: &str) -> Option<Answer> {
        let mut seats =  input.split_whitespace()
            .map(|line| {
                let (row, _) = &line[..7].chars()
                    .fold((0, 127), |(start, end), next| {
                        let halfway = (start + end) / 2;
                        match next {
                            'F' => (start, halfway),
                            'B' => (halfway + 1, end),
                            _ => { unreachable!( )}
                        }
                    });
                let (column, _) = &line[7..].chars()
                    .fold((0, 7), |(start, end), next| {
                        let halfway = (start + end) / 2;
                        match next {
                            'L' => (start, halfway),
                            'R' => (halfway + 1, end),
                            _ => unreachable!(),
                        }
                    });

                let ans: u64 = (row * 8) + column;
                ans
            }).collect::<Vec<u64>>();
            seats.sort();

            let my_seat = (seats[0]..=seats[seats.len() - 1] as u64)
                .zip(seats)
                .find(|(expected, seat)| expected != seat);
            
        Some(my_seat.unwrap().0)
    }
}

#[cfg(test)]
mod tests {
    use super::Day5;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = "FBFBBFFRLR";
        assert_eq!(Day5::part_1(test_input), Some(357));
    }

    #[test]
    #[ignore]
    fn part2_works() {
        // Not sure how to make this easily since it wasn't provided
        let test_input = "";
        assert_eq!(Day5::part_2(test_input), Some(0));
    }
}
