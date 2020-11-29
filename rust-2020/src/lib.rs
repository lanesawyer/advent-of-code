pub type Answer = String;

pub trait Day {
    fn run(day: &str) -> Answer;
}
pub struct Day1;

impl Day for Day1 {
    fn run(_input: &str) -> Answer {
        String::from("day 1")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Day, Day1};

    #[test]
    fn day1_works() {
        assert_eq!(Day1::run("1234"), "day 1");
    }
}
