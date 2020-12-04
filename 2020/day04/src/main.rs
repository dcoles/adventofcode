use std::path::Path;
use std::fs;
use std::collections::HashMap;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];  // ignore "cid"

fn main() {
    let passports = read_input("input.txt");

    println!("Part 1: Number of valid passports is {}", passports.iter().filter(|p| p.has_required_fields()).count());
    println!("Part 2: Number of valid passports is {}", passports.iter().filter(|p| p.is_valid()).count());
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Passport> {
    fs::read_to_string(path).expect("Failed to read input")
        .split("\n\n")
        .map(|s| Passport::from_str(s))
        .collect()
}

struct Passport {
    attributes: HashMap<String, String>,
}

impl Passport {
    fn from_str(s: &str) -> Self {
        let attributes = s.split_ascii_whitespace()
            .map(|kv| {
                let mut split = kv.split(":");
                let key = split.next().expect("Failed to parse attribute");
                let value = split.next().expect("Failed to parse attribute");

                (key.to_string(), value.to_string())
            }).collect();

        Passport { attributes }
    }

    fn has_required_fields(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|&f| self.attributes.contains_key(f))
    }

    fn is_valid(&self) -> bool {
        self.has_required_fields()
            && valid_byr(self.attributes.get("byr").unwrap())
            && valid_iyr(self.attributes.get("iyr").unwrap())
            && valid_eyr(self.attributes.get("eyr").unwrap())
            && valid_hgt(self.attributes.get("hgt").unwrap())
            && valid_hcl(self.attributes.get("hcl").unwrap())
            && valid_ecl(self.attributes.get("ecl").unwrap())
            && valid_pid(self.attributes.get("pid").unwrap())
    }
}

fn valid_byr(s: &str) -> bool {
    if let Some(yr) = s.parse::<u32>().ok() {
        yr >= 1920 && yr <= 2002
    } else {
        false
    }
}

fn valid_iyr(s: &str) -> bool {
    if let Some(yr) = s.parse::<u32>().ok() {
        yr >= 2010 && yr <= 2020
    } else {
        false
    }
}

fn valid_eyr(s: &str) -> bool {
    if let Some(yr) = s.parse::<u32>().ok() {
        yr >= 2020 && yr <= 2030
    } else {
        false
    }
}

fn valid_hgt(s: &str) -> bool {
    if let Some(height) = Height::from_str(s) {
        match height {
            Height::Centimeters(hgt) => hgt >= 150 && hgt <= 193,
            Height::Inches(hgt) => hgt >= 59 && hgt <= 76,
        }
    } else {
        false
    }
}

fn valid_hcl(hcl: &str) -> bool {
    hcl.starts_with("#") && hcl[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn valid_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn valid_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_numeric())
}

enum Height {
    Centimeters(u32),
    Inches(u32),
}

impl Height {
    fn from_str(s: &str) -> Option<Height> {
        let len = s.len();
        let value: u32 = s[..len-2].parse().ok()?;
        let suffix = &s[len-2..];
        match suffix {
            "cm" => Some(Height::Centimeters(value)),
            "in" => Some(Height::Inches(value)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let passports = read_input("input1.txt");
        assert_eq!(4, passports.len());
        assert_eq!(passports.iter().filter(|p| p.has_required_fields()).count(), 2);
    }

    #[test]
    fn test_byr() {
        assert!(valid_byr("2002"));
        assert!(!valid_byr("2003"));
    }

    #[test]
    fn test_hgt() {
        assert!(valid_hgt("60in"));
        assert!(valid_hgt("190cm"));
        assert!(!valid_hgt("190in"));
        assert!(!valid_hgt("190"));
    }

    #[test]
    fn test_invalid_passport1() {
        let passport = Passport::from_str("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926");
        assert!(!passport.is_valid());
    }

    #[test]
    fn test_invalid_passport2() {
        let passport = Passport::from_str("iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946");
        assert!(!passport.is_valid());
    }

    #[test]
    fn test_invalid_passport3() {
        let passport = Passport::from_str("hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277");
        assert!(!passport.is_valid());
    }

    #[test]
    fn test_invalid_passport4() {
        let passport = Passport::from_str("hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007");
        assert!(!passport.is_valid());
    }

    #[test]
    fn test_valid_passport1() {
        let passport = Passport::from_str("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f");
        assert!(passport.is_valid())
    }

    #[test]
    fn test_valid_passport2() {
        let passport = Passport::from_str("eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm");
        assert!(passport.is_valid())
    }

    #[test]
    fn test_valid_passport3() {
        let passport = Passport::from_str("hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022");
        assert!(passport.is_valid())
    }

    #[test]
    fn test_valid_passport4() {
        let passport = Passport::from_str("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719");
        assert!(passport.is_valid())
    }
}

