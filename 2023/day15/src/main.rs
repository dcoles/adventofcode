//! Advent of Code 2023: Day 15 "Lens Library"
//! https://adventofcode.com/2023/day/X

use std::borrow::Borrow;
use std::{fs, io};
use std::path::Path;

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
    let mut hashmap = HashMap::new();

    for step in &input.values {
        let (label, value) = if step.ends_with('-') {
            let label = step.strip_suffix('-').unwrap();

            (label.to_owned(), None)
        } else {
            let (label, value) = step.split_once('=').unwrap();

            (label.to_owned(), Some(value.parse::<u32>().unwrap()))
        };

        if let Some(value) = value {
            hashmap.insert(label, value);
        } else {
            hashmap.remove(label);
        }
    }

    hashmap.entries.into_iter()
    .enumerate()
    .map(|(i, entries)| {
        (i as u32 + 1) * entries.into_iter().enumerate().map(|(j, (_, val))| (j as u32 + 1) * val).sum::<u32>()
    }).sum()
}

#[derive(Debug, Clone)]
struct HashMap<T, V> where T: Hash + Eq {
    entries: Vec<Vec<(T, V)>>,
}

impl<T, V> HashMap<T, V> where T: Hash + Eq {
    const BOXES: usize = 256;

    fn new() -> Self {
        Self {
            entries: (0..Self::BOXES).map(|_| vec![]).collect(),
        }
    }

    fn insert(&mut self, key: T, value: V) {
        let hash = key.hash() as usize % Self::BOXES;
        if let Some(i) = self.entries[hash].iter().position(|(k, _)| *k == key) {
            self.entries[hash][i] = (key, value);
        } else {
            self.entries[hash].push((key, value));
        }
    }

    fn remove(&mut self, key: T) -> Option<V> {
        let hash = key.hash() as usize % Self::BOXES;
        if let Some(i) = self.entries[hash].iter().position(|(k, _)| *k == key) {
            Some(self.entries[hash].remove(i).1)
        } else {
            None
        }
    }
}

trait Hash {
    fn hash(&self) -> u64;
}

impl<T> Hash for T where T: Borrow<str> {
    fn hash(&self) -> u64 {
        self.borrow()
        .chars()
        .map(|c| c as u64)
        .fold(0, |acc, v| (acc + v) * 17)
    }
}

fn hash(s: &str) -> usize {
    let mut value = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        //value = value.rem(256);
    }

    value % 256
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
