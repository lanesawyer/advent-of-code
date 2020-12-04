use super::Answer;
use crate::Day;

pub struct Day4;

impl Day for Day4 {
    fn part_1(input: &str) -> Option<Answer> {
        let mut num_passports = 0;

        let mut iterator = input.split('\n');
        while let Some(line) = iterator.next() {
            println!("processing line {}", line);
            let mut passport = Passport::new(line);

            while let Some(next_line) = iterator.next() {
                println!("processing further lines {}", next_line);
                if !next_line.trim().is_empty() {
                    passport.parse_line(next_line);
                } else {
                    break;
                }
            }

            println!("{:?}", passport);

            if passport.is_valid() {
                num_passports += 1
            }
        }

        Some(num_passports)
    }

    fn part_2(_input: &str) -> Option<Answer> {
        todo!()
    }
}

#[derive(Default, Debug)]
struct Passport {
    pub byr: Option<u32>,    // (Birth Year)
    pub iyr: Option<u32>,    // (Issue Year)
    pub eyr: Option<u32>,    //(Expiration Year)
    pub hgt: Option<String>, //(Height)
    pub hcl: Option<String>, //(Hair Color)
    pub ecl: Option<String>, //(Eye Color)
    pub pid: Option<String>, //(Passport ID)
    pub cid: Option<String>, //(Country ID)
}

impl Passport {
    fn new(line: &str) -> Self {
        let mut new_passport: Passport = Default::default();
        new_passport.parse_line(line);
        new_passport
    }

    fn parse_line(&mut self, line: &str) {
        line.split(' ')
            .filter(|s| !s.is_empty())
            .map(|data_pair| {
                let data = data_pair.split(':').collect::<Vec<&str>>();
                let field = data[0];
                let value = data[1];
                (field, value)
            })
            .for_each(|pair| match pair {
                ("byr", value) => self.byr = value.parse::<u32>().ok(),
                ("iyr", value) => self.iyr = value.parse::<u32>().ok(),
                ("eyr", value) => self.eyr = value.parse::<u32>().ok(),
                ("hgt", value) => self.hgt = Some(value.into()),
                ("hcl", value) => self.hcl = Some(value.into()),
                ("ecl", value) => self.ecl = Some(value.into()),
                ("pid", value) => self.pid = Some(value.into()),
                ("cid", value) => self.cid = Some(value.into()),
                (_, _) => {}
            })
    }

    fn is_valid(&self) -> bool {
        let fields = [self.byr, self.iyr, self.eyr];
        let other_fields = [
            self.hgt.as_ref(),
            self.hcl.as_ref(),
            self.ecl.as_ref(),
            self.pid.as_ref(),
        ];

        let u32s_good = fields.iter().all(|field| field.is_some());
        let strings_good = other_fields.iter().all(|field| field.is_some());

        println!("{} && {}", u32s_good, strings_good);

        u32s_good && strings_good
    }
}

#[cfg(test)]
mod tests {
    use super::Day4;
    use crate::Day;

    #[test]
    fn part1_works() {
        let test_input = "
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
            
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
            
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
            
            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        ";
        assert_eq!(Day4::part_1(test_input), Some(2));
    }

    #[test]
    fn part2_works() {
        let test_input = "
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
            
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
            
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
            
            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        ";
        assert_eq!(Day4::part_2(test_input), Some(2));
    }
}
