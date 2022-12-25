//! Advent of Code 2022: Day 25
//! https://adventofcode.com/2022/day/25

use std::fmt::Display;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> String {
    let n: i64 = input.values.iter().map(|&s| -> i64 { s.into() }).sum();

    Snafu::new(n).to_string()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Snafu(i64);

impl Snafu {
    fn new(val: i64) -> Snafu {
        Self(val)
    }
}

impl From<Snafu> for i64 {
    fn from(value: Snafu) -> Self {
        value.0
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut val = self.0;
        let mut digits = Vec::new();

        while val != 0 {
            digits.push(val.rem_euclid(5));
            val = val.div_euclid(5);
        }

        for i in 0..digits.len() {
            if digits[i] > 2 {
                digits[i] -= 5;
                if i < digits.len() - 1 {
                    digits[i + 1] += 1;
                } else {
                    digits.push(1)
                }
            }
        }

        let repr: String = digits.iter().copied().rev().map(|d| {
            match d {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("malformed digits: {:?}", digits),
            }
        }).collect();

        write!(f, "{}", repr)
    }
}

impl FromStr for Snafu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = 0;

        for c in s.chars() {
            value *= 5;

            value += match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("Unknown digit: {:?}", c),
            };
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Snafu>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            let value: Snafu = line.parse().unwrap();
            values.push(value);
        }

        Ok(Input { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snafu_parse() {
        assert_eq!("1".parse::<Snafu>().unwrap().0, 1);
        assert_eq!("2".parse::<Snafu>().unwrap().0, 2);
        assert_eq!("1=".parse::<Snafu>().unwrap().0, 3);
        assert_eq!("1121-1110-1=0".parse::<Snafu>().unwrap().0, 314159265);
    }

    #[test]
    fn test_snafu_to_string() {
        assert_eq!(Snafu(1747).to_string(), "1=-0-2");
    }

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), "2=-1=0");
    }
}
