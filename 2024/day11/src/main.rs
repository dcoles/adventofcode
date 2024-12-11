//! Advent of Code 2024: Day 11
//! https://adventofcode.com/2024/day/11

use std::{fs, io};
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example2.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn digits(n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    (n.ilog10() + 1) as usize
}

fn part1(input: &Input) -> usize {
    // `slow` and `fast` give identical results.
    slow(&input.values, 25)
}

fn part2(input: &Input) -> usize {
    // `slow` and `fast` give identical results, but `slow` will never complete.
    fast(&input.values, 75)
}

fn slow(stones: &[usize], n: usize) -> usize {
    let mut stones = stones.iter().copied().collect();

    for _ in 0..n {
        let mut new_stones = Vec::new();

        for stone in stones {
            let digits = digits(stone);
            match stone {
                0 => { new_stones.push(1); },
                x if digits % 2 == 0 => {
                    let factor = 10usize.pow(digits as u32 / 2);
                    new_stones.push(x / factor);
                    new_stones.push(x % factor);
                },
                x => { new_stones.push(x * 2024); }
            }
        }

        stones = new_stones;
    }

    stones.len()
}

fn fast(stones: &[usize], n: usize) -> usize {
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
    for value in stones.iter().copied() {
        *counts.entry(value).or_default() += 1;
    }

    for _ in 0..n {
        let mut new_counts = BTreeMap::new();

        for (stone, n) in counts {
            let digits = digits(stone);
            match stone {
                0 => {
                    *new_counts.entry(1).or_default() += n;
                },
                x if digits % 2 == 0 => {
                    let factor = 10usize.pow(digits as u32 / 2);
                    *new_counts.entry(x / factor).or_default() += n;
                    *new_counts.entry(x % factor).or_default() += n;
                },
                x => {
                    *new_counts.entry(x * 2024).or_default() += n;
                }
            }
        }

        counts = new_counts
    }

    counts.values().sum()
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<usize>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.trim().split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 55312);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 222461);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 264350935776416);
    }
}
