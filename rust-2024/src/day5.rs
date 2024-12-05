use std::collections::HashMap;

use aoc_utils::{test_day, AdventError, Answer, Day};

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut do_rules = true;
        let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();

        let middle_total: u64 = input
            .lines()
            .filter_map(|line| {
                let line = line.trim();

                // Once we encounter an empty line, we are done processing rules
                // and can move to processing input
                if line.is_empty() {
                    do_rules = false;
                    return None;
                }

                if do_rules {
                    let mut parts = line.split('|');
                    let page = parts.next().unwrap().parse::<u64>().unwrap();
                    let page_after = parts.next().unwrap().parse::<u64>().unwrap();

                    rules.entry(page).or_default().push(page_after);
                } else {
                    println!("process: {:?}", line);
                    let parts: Vec<u64> = line
                        .split(',')
                        .map(|part| part.parse::<u64>().unwrap())
                        .collect();
                    let passed = parts.iter().enumerate().all(|(index, part)| {
                        // check the rules!
                        println! {"part: {:?}", part};
                        let after_pages = rules.get(part);

                        // If we have no rules, make sure it's the last item in the list, otherwise
                        // we have failed!
                        if after_pages.is_none() {
                            return index == parts.len() - 1;
                        }
                        println!("after_pages: {:?}", after_pages);
                        let remaining = parts.iter().skip(index + 1).all(|p: &u64| {
                            println!("check for {}", p);
                            after_pages.unwrap().contains(p)
                        });
                        remaining
                    });

                    if passed {
                        println!("------ passed!");
                        let wat = parts.get(parts.len() / 2).unwrap_or(&0);
                        return Some(*wat);
                    }

                    println!("----- failed!");
                    return None;
                }
                None
            })
            .sum();

        Ok(middle_total)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let total = 1;
        Ok(total)
    }
}

test_day!(
    Day5,
    143,
    123,
    r#"47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47"#
);
