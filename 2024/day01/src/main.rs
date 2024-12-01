//! Advent of Code 2024: Day 1
//! https://adventofcode.com/2024/day/1

use std::{fs, io};
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    for (l, r) in input.values.iter().cloned() {
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let mut total_distance = 0;
    for (l, r) in left.into_iter().zip(right.into_iter()) {
        // Calculate distance between pairs
        total_distance += l.abs_diff(r);
    }

    total_distance
}

fn part2(input: &Input) -> usize {
    let mut left = Vec::new();
    let mut right: HashMap<_, usize> = HashMap::new();

    for (l, r) in input.values.iter().cloned() {
        left.push(l);
        *right.entry(r).or_default() += 1;
    }

    let mut similarity_score = 0;
    for l in left {
        let count = right.get(&l).copied().unwrap_or_default();

        // Calculate similarity score
        similarity_score += l * count;
    }

    similarity_score
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(usize, usize)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|s| {
            let mut split = s.split_ascii_whitespace();
            let a = split.next().unwrap().parse().unwrap();
            let b = split.next().unwrap().parse().unwrap();

            (a, b)
        }).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1722302);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 31);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 20373490);
    }
}
