//! Advent of Code 2023: Day 9
//! https://adventofcode.com/2023/day/9

use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Vec<i64>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>()
        }).collect();

        Ok(Self { values })
    }
}

fn part1(input: &Input) -> i64 {
    let mut prediction = vec![];
    for sequence in &input.values {
        // Process history
        let mut deltas = vec![sequence.clone()];
        while deltas.last().unwrap().iter().any(|&n| n != 0) {
            let last = deltas.last().unwrap();
            let delta = last.windows(2).map(|s| s[1] - s[0]).collect();
            deltas.push(delta);
        }

        // Make predictions
        let mut new = vec![0];
        for delta in deltas.iter().rev().skip(1) {
            let cur = *delta.last().unwrap();
            let d = *new.last().unwrap();
            new.push(cur + d);
        }

        prediction.push(*new.last().unwrap());
    }

    prediction.into_iter().sum()
}

fn part2(input: &Input) -> i64 {
    let mut prediction = vec![];
    for sequence in &input.values {
        // Process history
        let mut deltas = vec![sequence.clone()];
        while deltas.last().unwrap().iter().any(|&n| n != 0) {
            let last = deltas.last().unwrap();
            let delta = last.windows(2).map(|s| s[1] - s[0]).collect();
            deltas.push(delta);
        }

        // Make predictions
        let mut new = vec![0];
        for delta in deltas.iter().rev().skip(1) {
            let cur = *delta.first().unwrap();
            let d = *new.last().unwrap();
            new.push(cur - d);
        }

        prediction.push(*new.last().unwrap());
    }

    prediction.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 114);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1725987467);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 2);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 971);
    }
}
