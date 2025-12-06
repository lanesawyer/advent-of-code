use std::collections::HashMap;

use aoc_utils::{AdventError, Answer, Day, test_day};

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
                    let parts: Vec<u64> = line
                        .split(',')
                        .map(|part| part.parse::<u64>().unwrap())
                        .collect();

                    let is_ordered = parts.iter().enumerate().all(|(index, part)| {
                        // check the rules!
                        let after_pages = rules.get(part);

                        // If we have no rules, make sure it's the last item in the list,
                        // (as any last items don't necessarily have rules), and if
                        // it's not the end, then something is out of order!
                        if after_pages.is_none() {
                            return index == parts.len() - 1;
                        }

                        parts
                            .iter()
                            .skip(index + 1)
                            .all(|p: &u64| after_pages.unwrap().contains(p))
                    });

                    if is_ordered {
                        let mid_point = parts.get(parts.len() / 2).unwrap_or(&0);
                        return Some(*mid_point);
                    }

                    return None;
                }
                None
            })
            .sum();

        Ok(middle_total)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
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
                    let parts: Vec<u64> = line
                        .split(',')
                        .map(|part| part.parse::<u64>().unwrap())
                        .collect();
                    let passed = parts.iter().enumerate().all(|(index, part)| {
                        // check the rules!
                        let after_pages = rules.get(part);

                        // If we have no rules, make sure it's the last item in the list,
                        // (as any last items don't necessarily have rules), and if
                        // it's not the end, then something is out of order!
                        if after_pages.is_none() {
                            return index == parts.len() - 1;
                        }

                        parts
                            .iter()
                            .skip(index + 1)
                            .all(|p: &u64| after_pages.unwrap().contains(p))
                    });

                    // Now we don't care about passing ones, we want to fix bad ones!
                    if passed {
                        return None;
                    }

                    // Loop until we get a list that passes the rules checks
                    let mut unordered = true;
                    let mut reordered_parts = parts.clone();

                    while unordered {
                        // Loop through the page orders, check if it's ordered, and move it around if not
                        'outer: for (index, part) in reordered_parts.clone().iter().enumerate() {
                            let after_pages = rules.get(part);

                            // We did it! At the end and no problems, we're done!
                            if index == parts.len() - 1 {
                                unordered = false;
                                break;
                            }

                            // If we don't have any rules and we're not already at the end of the list,
                            // we can assume it's the last page
                            if after_pages.is_none() {
                                reordered_parts.remove(index);
                                reordered_parts.push(*part);
                                break;
                            }

                            // Check the number, if anything after it in the list does not appear in its
                            // rules, then the number it doesn't contain is definitely before it
                            for (i, p) in reordered_parts.clone().iter().enumerate().skip(index + 1)
                            {
                                let is_there = after_pages.unwrap().iter().position(|x| x == p);
                                if is_there.is_none() {
                                    reordered_parts.remove(i);
                                    reordered_parts.insert(i - 1, *p);
                                    break 'outer;
                                }
                            }
                        }
                    }

                    let mid_point = reordered_parts.get(parts.len() / 2).unwrap_or(&0);
                    return Some(*mid_point);
                }
                None
            })
            .sum();

        Ok(middle_total)
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
