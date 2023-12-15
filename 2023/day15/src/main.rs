//! Advent of Code 2023: Day 15 "Lens Library"
//! https://adventofcode.com/2023/day/X

use std::ops::Rem;
use std::{fs, io};
use std::path::Path;

const N_BOXES: usize = 256;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    input.values.iter().map(|s| hash(&s)).sum()
}

fn part2(input: &Input) -> u32 {
    let mut boxes: Vec<Vec<(String, u32)>> = (0..N_BOXES).map(|_| vec![]).collect();

    for step in &input.values {
        let (label, value) = if step.ends_with('-') {
            let label = step.strip_suffix('-').unwrap();

            (label.to_owned(), None)
        } else {
            let (label, value) = step.split_once('=').unwrap();

            (label.to_owned(), Some(value.parse::<u32>().unwrap()))
        };

        let h = hash(&label);
        if let Some(value) = value {
            if let Some(i) = find(&boxes[h], &label) {
                boxes[h][i] = (label, value);
            } else {
                boxes[h].push((label, value));
            }
        } else {
            if let Some(i) = find(&boxes[h], &label) {
                boxes[h].remove(i);
            }
        }
    }

    boxes.into_iter()
    .enumerate()
    .map(|(i, entries)| {
        (i as u32 + 1) * entries.into_iter().enumerate().map(|(j, (_, val))| (j as u32 + 1) * val).sum::<u32>()
    }).sum()
}

fn find(values: &[(String, u32)], key: &str) -> Option<usize> {
    values.iter().position(|(k, _)| *k == key)
}

fn hash(s: &str) -> usize {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value = value.rem(256);
    }

    value
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<String>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.trim().split(",").map(|s| s.to_owned()).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 1320);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 518107);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 145);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 303404);
    }
}
