//! Advent of Code 2021: Day 25
//! https://adventofcode.com/2021/day/25

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file("day25/input.txt").expect("failed to read input");
    println!("{:?}", input);

    // Part 1
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> usize {
    0
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
    fn test_part1() {
        todo!();
    }
}
