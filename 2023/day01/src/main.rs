//! Advent of Code 2023: Day 01
//! https://adventofcode.com/2023/day/01

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2 (take 1): {}", part2_take1(&input));
    println!("Part 2 (take 2): {}", part2_take2(&input));
}

fn part1(input: &Input) -> u32 {
    let mut values = Vec::new();
    for line in &input.values {
        let chars: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();

        let a = chars.first().unwrap().to_digit(10).unwrap();
        let b = chars.last().unwrap().to_digit(10).unwrap();

        values.push(a * 10 + b);
    }

    values.iter().sum()
}

fn part2_take1(input: &Input) -> u32 {
    // We use a non-greedy wildcard match at the *start*, so we capture the first valid number
    let first_re = regex::Regex::new(r"^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*$").unwrap();

    // We use a non-greedy wildcard match at the *end*, so we capture the last valid number
    let last_re = regex::Regex::new(r"^.*(\d|one|two|three|four|five|six|seven|eight|nine).*?$").unwrap();

    let mut values = Vec::new();
    for line in &input.values {
        let first_match = first_re.captures(&line).unwrap();
        let last_match = last_re.captures(&line).unwrap();

        let a = to_digit(&first_match[1]).unwrap();
        let b = to_digit(&last_match[1]).unwrap();

        values.push(a * 10 + b);
    }

    values.iter().sum()
}

static DIGITS: [(&str, u32); 9] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

fn part2_take2(input: &Input) -> u32 {
    let mut values = Vec::new();

    for line in &input.values {
        let mut first = 0;
        for i in 0..line.len() {
            if let Some(n) = match_digit(line, i) {
                first  = n;
                break;
            }
        }

        let mut last = 0;
        for i in (0..line.len()).rev() {
            if let Some(n) = match_digit(line, i) {
                last = n;
                break;
            }
        }

        values.push(10 * first + last);
    }

    values.into_iter().sum()
}

fn match_digit(line: &str, offset: usize) -> Option<u32> {
    // Is this an ordinary numeric digit?
    if let Ok(n) = line[offset..offset+1].parse() {
        return Some(n);
    }

    // Is this the word form of a digit?
    for &(digit, n) in &DIGITS {
        let digit_end = (offset + digit.len()).min(line.len());
        if &line[offset..digit_end] == digit {
            return Some(n);
        }
    }

    None
}

/// Convert a string to the number it represents.
fn to_digit(s: &str) -> Option<u32> {
    let digit = match s {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => return None,
    };

    Some(digit)
}


#[derive(Debug, Clone)]
struct Input {
    values: Vec<String>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            values.push(line.to_string());
        }

        Ok(Input { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 142);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 55607);
    }

    #[test]
    fn test_part2_example() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2_take1(&input), 281);
        assert_eq!(part2_take2(&input), 281);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2_take1(&input), 55291);
        assert_eq!(part2_take2(&input), 55291);
    }
}
