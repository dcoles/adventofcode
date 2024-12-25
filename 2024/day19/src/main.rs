//! Advent of Code 2024: Day 19
//! https://adventofcode.com/2024/day/19

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

fn main() {
    //let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut count = 0;

    for design in &input.designs {

        let mut edge: BTreeSet<usize> = [0].into_iter().collect();
        while let Some(pos) = edge.pop_first() {
            if pos == design.len() {
                count += 1;
                break;
            }

            for pattern in input.patterns.iter() {
                if design[pos..].starts_with(pattern.as_str()) {
                    edge.insert(pos + pattern.len());
                }
            }
        }
    }

    count
}

fn part2(input: &Input) -> usize {
    let mut count = 0;

    for design in &input.designs {
        let mut multiplier: BTreeMap<usize, usize> = [(0, 1)].into_iter().collect();

        for pos in 0..design.len() {
            if let Some(mul) = multiplier.get(&pos).copied() {
                let mut match_lengths: BTreeMap<usize, usize> = BTreeMap::new();
                for pattern in input.patterns.iter() {
                    if design[pos..].starts_with(pattern.as_str()) {
                        *match_lengths.entry(pattern.len()).or_default() += 1;
                    }
                }

                for (len, count) in match_lengths {
                    *multiplier.entry(pos + len).or_default() += mul * count;
                }
            }
        }

        if let Some(&mul) = multiplier.get(&design.len()) {
            count += mul;
        }
    }

    count
}

#[derive(Debug, Clone)]
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let (chunk1, chunk2) = input.split_once("\n\n").unwrap();
        let patterns = chunk1.trim().split(", ").map(str::to_string).collect();
        let designs = chunk2.lines().map(|l| l.trim().to_string()).collect();

        Ok(Self { patterns, designs })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 6);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 311);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 16);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 616234236468263);
    }
}
