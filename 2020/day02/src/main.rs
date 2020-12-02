use std::path::Path;
use std::fs;
use regex::Regex;


fn main() {
    let passwords = read_input("input.txt");

    // Part 1
    println!("Part 1: Number of valid passwords is {}", passwords.iter().filter(|p| p.valid()).count());
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Password> {
    let mut passwords = Vec::new();
    let regex= Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    for line in fs::read_to_string(path).expect("Failed to read input").lines() {
        if line.is_empty() {
            continue;
        }

        let m = regex.captures(line).expect("Failed to parse input");
        let min = m[1].parse().unwrap();
        let max = m[2].parse().unwrap();
        let character = m[3].chars().next().unwrap();
        let password = m[4].into();

        passwords.push(Password { min, max, character, password });
    }

    passwords
}

#[derive(Debug)]
struct Password {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl Password {
    fn valid(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.character).count();

        count >= self.min && count <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let passwords = read_input("input1.txt");
        assert_eq!(passwords.iter().filter(|p| p.valid()).count(), 2);
    }
}

