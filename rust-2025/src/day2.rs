use aoc_utils::{AdventError, Answer, Day, test_day};

pub struct Day2;

impl Day for Day2 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let groups = get_groups(input.trim());
        let result = groups.map(part_1_find_silly_patterns).flatten().sum::<u64>();

        Ok(result)
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let groups = get_groups(input.trim());
        Ok(0)
    }
}

fn get_groups(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.trim().split(',').map(|ranges| {
        let mut range_nums = ranges.split('-');
        let start = range_nums
            .next()
            .expect("Missing start")
            .parse::<u64>()
            .expect("Invalid number");
        let end = range_nums
            .next()
            .expect("Missing end")
            .parse::<u64>()
            .expect("Invalid number");
        (start, end)
    })
}

fn part_1_find_silly_patterns((start, end): (u64, u64)) -> impl Iterator<Item = u64>{
   (start..=end).filter_map(|number| {
        let stringed = number.to_string();
        let start_len = stringed.len();
        let length_of_silly_pattern = start_len / 2;

        let first_half = &stringed[0..length_of_silly_pattern];
        let second_half = &stringed[length_of_silly_pattern..start_len];

        if first_half == second_half {
            Some(number)
        } else {
            None
        }
   })
}

// fn part_2_find_silly_patterns((start, end): (u64, u64)) -> Vec<u64> {
//    (start..=end).filter_map(|number| {
        
//    }).collect()
// }

test_day!(
    Day2,
    1227775554,
    6,
    r#"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
    "#
);
