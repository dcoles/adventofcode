use std::path::Path;
use std::fs;
use std::collections::HashMap;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];  // ignore "cid"

fn main() {
    let passports = read_input("input.txt");

    println!("Part 1: Number of valid passports is {}", passports.iter().filter(|p| p.is_valid()).count());
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

    fn is_valid(&self) -> bool {
        REQUIRED_FIELDS.iter().all(|&f| self.attributes.contains_key(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let passports = read_input("input1.txt");
        assert_eq!(4, passports.len());
        assert_eq!(passports.iter().filter(|p| p.is_valid()).count(), 2);
    }
}

