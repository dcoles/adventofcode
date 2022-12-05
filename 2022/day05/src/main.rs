//! Advent of Code 2022: Day 5
//! https://adventofcode.com/2022/day/5

use std::fs;
use std::io;
use std::path::Path;

use regex::Regex;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> String {
    let mut crates = input.crates.clone();

    for step in &input.steps {
        for _ in 0..step.count {
            // Crates are moved one at a time.
            let c = crates[step.from - 1].pop().expect("tried to move missing crate");
            crates[step.to - 1].push(c);
        }
    }

    // After the rearrangement procedure completes, what crate ends up on top of each stack?
    crates.iter().filter_map(|s| s.last()).collect()
}

fn part2(input: &Input) -> String {
    let mut crates = input.crates.clone();

    for step in &input.steps {
        let bottom = crates[step.from - 1].len() - step.count;

        // Pick up and move multiple crates at once.
        let carried: Vec<_> = crates[step.from - 1].drain(bottom..).collect();
        crates[step.to - 1].extend(carried);
    }

    // After the rearrangement procedure completes, what crate ends up on top of each stack?
    crates.iter().filter_map(|s| s.last()).collect()
}

#[derive(Debug, Clone)]
struct Input {
    crates: Vec<Vec<char>>,
    steps: Vec<Move>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let (chunk1, chunk2) = input.split_once("\n\n").expect("failed to split chunks");

        // Parse crate structure
        let mut crates = vec![vec![];9];
        for line in chunk1.lines().rev().skip(1) {
            let chars: Vec<char> = line.chars().collect();
            for (i, c) in chars.into_iter().skip(1).step_by(4).enumerate() {
                if c.is_alphabetic() {
                    crates[i].push(c);
                }
            }
        }

        // Parse rearrangement procedure
        let re = Regex::new(r"move (\d+) from (\d)+ to (\d)+").unwrap();
        let mut steps = Vec::new();
        for cap in re.captures_iter(chunk2) {
            let count = cap[1].parse().unwrap();
            let from = cap[2].parse().unwrap();
            let to = cap[3].parse().unwrap();

            steps.push(Move { count, from, to })
        }

        Ok(Input { crates, steps })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), "CMZ")
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), "MCD")
    }
}
