use super::Answer;
use crate::Day;

pub struct Day4;

impl Day for Day4 {
    fn part_1(input: &str) -> Option<Answer> {
        Some(count_passports(input, false))
    }

    fn part_2(input: &str) -> Option<Answer> {
        Some(count_passports(input, true))
    }
}

fn count_passports(input: &str, strict_validation: bool) -> Answer {
    let mut num_passports = 0;

    let mut iterator = input.split('\n');
    while let Some(line) = iterator.next() {
        let mut passport = Passport::new(line);

        while let Some(next_line) = iterator.next() {
            if !next_line.trim().is_empty() {
                passport.parse_line(next_line);
            } else {
                break;
            }
        }

        let is_valid = if strict_validation {
            passport.is_valid_strict()
        } else {
            passport.is_valid()
        };

        if is_valid {
            num_passports += 1
        }
    }

    num_passports
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
        let num_fields = [self.byr, self.iyr, self.eyr];
        let string_fields = [
            self.hgt.as_ref(),
            self.hcl.as_ref(),
            self.ecl.as_ref(),
            self.pid.as_ref(),
        ];

        let u32s_good = num_fields.iter().all(|field| field.is_some());
        let strings_good = string_fields.iter().all(|field| field.is_some());

        u32s_good && strings_good
    }

    fn is_valid_strict(&self) -> bool {
        let byr_valid = match self.byr {
            Some(year) => year >= 1920 && year <= 2002,
            None => false,
        };

        let iyr_valid = match self.iyr {
            Some(year) => year >= 2010 && year <= 2020,
            None => false,
        };

        let eyr_valid = match self.eyr {
            Some(year) => year >= 2020 && year <= 2030,
            None => false,
        };

        let hgt_valid = match &self.hgt {
            Some(height) => {
                let value = &height[..height.len() - 2];

                if height.ends_with("cm") {
                    let number = value.parse::<u32>().unwrap_or(0);
                    number >= 150 && number <= 193
                } else if height.ends_with("in") {
                    let value = &height[..height.len() - 2];
                    let number = value.parse::<u32>().unwrap_or(0);
                    number >= 59 && number <= 76
                } else {
                    false
                }
            }
            None => false,
        };

        let hcl_valid = match &self.hcl {
            Some(hcl) => hcl.starts_with('#') && hcl.len() == 7,
            None => false,
        };

        let valid_ecls: Vec<String> = vec![
            "amb".into(),
            "blu".into(),
            "brn".into(),
            "gry".into(),
            "grn".into(),
            "hzl".into(),
            "oth".into(),
        ];
        let ecl_valid = match &self.ecl {
            Some(ecl) => valid_ecls.contains(&ecl),
            None => false,
        };

        let pid_valid = match &self.pid {
            Some(pid) => pid.len() == 9 && pid.parse::<u32>().is_ok(),
            None => false,
        };

        byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid & pid_valid
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
    fn part2_invalid_passports_works() {
        let test_input = "
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        ";
        assert_eq!(Day4::part_2(test_input), Some(0));
    }

    #[test]
    fn part2_valid_passports_works() {
        let test_input = "
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f
            
            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
            
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022
            
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ";
        assert_eq!(Day4::part_2(test_input), Some(4));
    }
}
