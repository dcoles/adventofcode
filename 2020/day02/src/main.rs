use std::path::Path;
use std::fs;
use regex::Regex;

fn main() {
    let passwords = read_input("input.txt");

    // Part 1
    println!("Part 1: Number of valid passwords is {}", passwords.iter().filter(|p| p.valid1()).count());

    // Part 2
    println!("Part 2: Number of valid passwords is {}", passwords.iter().filter(|p| p.valid2()).count());
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Password> {
    let mut passwords = Vec::new();
    let regex= Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    for line in fs::read_to_string(path).expect("Failed to read input").lines() {
        if line.is_empty() {
            continue;
        }

        let m = regex.captures(line).expect("Failed to parse input");
        let a = m[1].parse().unwrap();
        let b = m[2].parse().unwrap();
        let character = m[3].chars().next().unwrap();
        let password = m[4].into();

        passwords.push(Password { a, b, character, password });
    }

    passwords
}

#[derive(Debug)]
struct Password {
    a: usize,
    b: usize,
    character: char,
    password: String,
}

impl Password {
    /// Policy 1: Password must contain `character` a minimum of `a` times and a maximum of `b` times.
    fn valid1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.character).count();

        count >= self.a && count <= self.b
    }

    /// Policy 2: Password must contain `character` at either index `a` or index `b`, but not both.
    /// Indexing is 1-based (there is no "index 0")
    fn valid2(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();

        (chars[self.a - 1] == self.character) ^ (chars[self.b - 1] == self.character)
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

    #[test]
    fn test_part2_example1() {
        let passwords = read_input("input1.txt");
        assert_eq!(passwords.iter().filter(|p| p.valid2()).count(), 1);
    }
}

