//! Advent of Code 2025: Day 2
//! <https://adventofcode.com/2025/day/2>

use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::{fs, io};
use std::path::Path;

// Largest number of digits in input values
const MAX_DIGITS: usize = 10;
const MAX_N: usize = 10usize.pow(MAX_DIGITS as u32 / 2) - 1;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    let mut max = 0;
    for range in &input.values {
        max = max.max(*range.end());
    }

    println!("max: {max}");

    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut sum = 0;

    for n in 1..MAX_N {
        let id: usize = format!("{n}{n}").parse().unwrap();
        for range in &input.values {
            if range.contains(&id) {
                sum += id;
            }
        }
    }

    sum
}

fn part2(input: &Input) -> usize {
    let mut sum = 0;

    let mut seen = HashSet::new();

    for n in 1..MAX_N {
        let mut s = format!("{n}{n}");

        while s.len() <= MAX_DIGITS {
            let id: usize = s.parse().unwrap();

            // Make sure we don't double-count
            if !seen.contains(&id) {
                for range in &input.values {
                    if range.contains(&id) {
                        sum += id;
                    }
                }
            }

            seen.insert(id);

            s = format!("{s}{n}");
        }
    }


    sum
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<RangeInclusive<usize>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let values = input.trim_ascii()
            .split(',')
            .map(|range| {
                let (first, last) = range.split_once('-').unwrap();
                let (first, last) = (first.parse().unwrap(), last.parse().unwrap());

                RangeInclusive::new(first, last)
            })
            .collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();


        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 24157613387);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 4174379265);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 33832678380);
    }
}
