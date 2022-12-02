use std::collections::HashMap;

use super::{AdventError, Answer};
use crate::Day;

pub struct Day7;

impl Day for Day7 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let bag_to_find = "shiny gold bag";

        let bag_contains_list = input
            .split("\n")
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts = line.split(" contain ").collect::<Vec<&str>>();
                let bag_name = parts[0].replace("bags", "bag");

                let contains_rules = parts[1]
                    .split(", ")
                    .map(|rule| {
                        // Default to 0 bags if we have a parsing error
                        // since that's the "no other bags." rule
                        let num = rule[..1].parse::<usize>().unwrap_or(0);

                        // Ditch the spaces, commas, bag vs bags
                        let name = rule[1..]
                            .trim()
                            .replace("bags", "bag") // bags
                            .replace(".", ""); // bag.

                        (num, name)
                    })
                    .collect::<Vec<(usize, String)>>();

                (bag_name, contains_rules)
            })
            .collect::<HashMap<String, Vec<(usize, String)>>>();

        let something = bag_contains_list.get(bag_to_find).unwrap();

        for pair in something {
            let (num, bag) = pair;

            bag_contains_list.get(bag);
        }
        println!("{:?}", something);
        // first thoughts
        // parse each line
        // track what it's named and what it can contain
        // take the input
        // find all that can contain that one
        // work your way up to what can contain that bag, etc.
        // recursively or whatever
        Ok(1)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        Ok(1)
    }
}

// fn find_bag() -> usize {
//     let something = bag_contains_list.get(bag_to_find);

//     for pair in something {
//         let (num, bag) = pair;

//         return num + find_bag()
//     }
// }

#[cfg(test)]
mod tests {
    use super::Day7;
    use crate::Day;

    #[test]
    fn part7_works() {
        let test_input = r#"7727
            979
            366
            299
            675
            7456"#;
        assert_eq!(Day7::part_1(test_input).unwrap(), 574579);
    }

    #[test]
    fn part2_works() {
        let test_input = r#"7727
            979
            366
            299
            675
            7456"#;
        assert_eq!(Day7::part_2(test_input).unwrap(), 247867950);
    }
}
