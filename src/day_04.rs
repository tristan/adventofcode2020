use std::{fs, io::{BufReader, BufRead}};
use adventofcode2020::ReadError;

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        self.byr.is_some() &&
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }

    fn validate_byr(&self) -> bool {
        self.byr.as_ref().map_or(None, |byr| byr.parse::<u16>().ok())
            .map_or(false, |byr| byr >= 1920 && byr <= 2002)
    }

    fn validate_iyr(&self) -> bool {
        self.iyr.as_ref().map_or(None, |iyr| iyr.parse::<u16>().ok())
            .map_or(false, |iyr| iyr >= 2010 && iyr <= 2020)
    }

    fn validate_eyr(&self) -> bool {
        self.eyr.as_ref().map_or(None, |eyr| eyr.parse::<u16>().ok())
            .map_or(false, |eyr| eyr >= 2020 && eyr <= 2030)
    }

    fn validate_hgt(&self) -> bool {
        self.hgt.as_ref().map_or(None, |hgt| {
            let suffix = &hgt[hgt.len() - 2..];
            match suffix {
                "in" => {
                    hgt[..hgt.len() - 2]
                        .parse::<u16>()
                        .ok()
                        .map(|hgt| hgt >= 59 && hgt <= 76)
                },
                "cm" => {
                    hgt[..hgt.len() - 2]
                        .parse::<u16>()
                        .ok()
                        .map(|hgt| hgt >= 150 && hgt <= 193)
                },
                _ => None
            }
        }).unwrap_or(false)
    }

    fn validate_hcl(&self) -> bool {
        self.hcl.as_ref().map_or(false, |hcl| {
            let mut hcli = hcl.chars();
            if hcl.len() == 7 && hcli.next() == Some('#') {
                hcli.filter(|&c| c.is_ascii_hexdigit())
                    .count() == 6
            } else {
                false
            }
        })
    }

    fn validate_ecl(&self) -> bool {
        self.ecl.as_ref().map(|ecl| {
            match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false
            }
        }).unwrap_or(false)
    }

    fn validate_pid(&self) -> bool {
        self.pid.as_ref().map_or(false, |pid| {
            if pid.len() == 9 {
                pid.chars().all(|c| c.is_ascii_digit())
            } else {
                false
            }
        })
    }

    fn is_valid_part2(&self) -> bool {
        self.validate_byr() &&
        self.validate_iyr() &&
        self.validate_eyr() &&
        self.validate_hgt() &&
        self.validate_hcl() &&
        self.validate_ecl() &&
        self.validate_pid()
    }

}

fn read_input<R>(reader: R) -> Result<Vec<Passport>, ReadError>
where R: BufRead
{
    let mut passports = vec![];

    let mut passport = Some(Passport::default());
    reader.lines().enumerate().map(|(line_no, line)| {
        let line_no = line_no + 1;
        let line: String = line
            .map_err(|e| ReadError::IoError(Some(line_no), e))?;

        if line.len() == 0 {
            passports.push(passport.take().unwrap());
            passport = Some(Passport::default());
        } else {
            let passport = passport.as_mut().unwrap();
            line.split(" ").map(|field| {
                let mut splitn = field.splitn(2, ":");
                let key = splitn.next().ok_or_else(|| ReadError::ParseError(line_no, field.to_string()))?;
                let value = splitn.next().ok_or_else(|| ReadError::ParseError(line_no, field.to_string()))?;
                match key {
                    "byr" => passport.byr = Some(value.to_string()),
                    "iyr" => passport.iyr = Some(value.to_string()),
                    "eyr" => passport.eyr = Some(value.to_string()),
                    "hgt" => passport.hgt = Some(value.to_string()),
                    "hcl" => passport.hcl = Some(value.to_string()),
                    "ecl" => passport.ecl = Some(value.to_string()),
                    "pid" => passport.pid = Some(value.to_string()),
                    "cid" => passport.cid = Some(value.to_string()),
                    _ => return Err(ReadError::ParseError(line_no, field.to_string()))
                }
                Ok(())
            }).collect::<Result<_, ReadError>>()?;
        }

        Ok(())
    }).collect::<Result<_, ReadError>>()?;
    if let Some(passport) = passport {
        passports.push(passport);
    }

    Ok(passports)
}

fn read_input_file(filename: &str) -> Result<Vec<Passport>, ReadError> {
    let file = fs::File::open(filename)
        .map_err(|e| ReadError::IoError(None, e))?;
    let reader = BufReader::new(file);
    read_input(reader)
}

fn part1(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_part1()).count()
}

fn part2(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid_part2()).count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let passports = read_input_file("day_04_input.txt")?;
    let result = part1(&passports);
    println!("part1: {}", result);
    let result = part2(&passports);
    println!("part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        let passports = read_input(input.as_bytes())?;
        assert_eq!(passports.len(), 4);

        assert!(passports[0].is_valid_part1());
        assert!(!passports[1].is_valid_part1());
        assert!(passports[2].is_valid_part1());
        assert!(!passports[3].is_valid_part1());

        assert_eq!(part1(&passports), 2);

        Ok(())
    }

    #[test]
    fn test_part2_invalid() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

        let passports = read_input(input.as_bytes())?;
        assert_eq!(passports.len(), 4);
        assert_eq!(part2(&passports), 0);

        Ok(())
    }

    #[test]
    fn test_part2_valid() -> Result<(), Box<dyn std::error::Error>> {
        let input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
        let passports = read_input(input.as_bytes())?;
        assert_eq!(passports.len(), 4);
        assert_eq!(part2(&passports), 4);

        Ok(())
    }
}
