pub type Answer = u32;

pub trait Day {
    fn part_1(day: &str) -> Option<Answer>;
    fn part_2(day: &str) -> Option<Answer>;
}

pub mod day1;
pub mod day2;
