//! Advent of Code 2022: Day 4
//! https://adventofcode.com/2022/day/4

use std::fs;
use std::io;
use std::ops::RangeInclusive;
use std::path::Path;

fn main() {
    let input = Input::from_file("day04/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    // In how many assignment pairs does one range fully contain the other?
    input.values.iter()
        .filter(|(r1, r2)| range_contains(r1, r2) || range_contains(r2, r1))
        .count()
}

/// Does one range contain another.
fn range_contains(range: &RangeInclusive<usize>, other: &RangeInclusive<usize>) -> bool {
    range.start() <= other.start() && range.end() >= other.end()
}

fn part2(input: &Input) -> usize {
    // In how many assignment pairs do the ranges overlap?
    input.values.iter()
        .filter(|(r1, r2)| range_overlaps(r1, r2))
        .count()
}

/// Do two ranges overlap at all?
fn range_overlaps(range: &RangeInclusive<usize>, other: &RangeInclusive<usize>) -> bool {
    range.end() >= other.start() && range.start() <= other.end()
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            let (a, b) = line.split_once(',').unwrap();

            let a = parse_range(a);
            let b = parse_range(b);
            values.push((a, b));
        }

        Ok(Input { values })
    }
}

/// Parse an inclusive range of the format `start-end`.
fn parse_range(s: &str) -> RangeInclusive<usize> {
    let (s1, s2) = s.split_once('-').unwrap();
    let (s1, s2) = (s1.parse().expect("unable to parse range start"), s2.parse().expect("unable to parse range end"));

    s1..=s2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 4);
    }
}
