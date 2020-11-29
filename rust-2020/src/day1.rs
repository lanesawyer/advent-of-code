use crate::Day;

pub struct Day1;

impl Day for Day1 {
    fn run(_input: &str) -> String {
        String::from("day 1")
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::Day1;

    #[test]
    fn works() {
        assert_eq!(Day1::run("1234"), "day 1");
    }
}
