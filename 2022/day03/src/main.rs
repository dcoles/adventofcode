//! Advent of Code 2022: Day 3
//! https://adventofcode.com/2022/day/3

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file("day03/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> u32 {
    let mut common: Vec<char> = Vec::new();

    for rucksack in &input.values {
        let n = rucksack.len();
        let first: HashSet<char> = rucksack.chars().take(n/2).collect();
        let second: HashSet<char> = rucksack.chars().skip(n/2).collect();

        // Find the item type that appears in both compartments of each rucksack.
        common.extend(&first & &second);
    }

    // What is the sum of the priorities of those item types?
    common.iter().map(|&item| priority(item)).sum::<u32>()
}

fn part2(input: &Input) -> u32 {
    let mut badges = Vec::new();

    // For safety, the Elves are divided into groups of three.
    for group in input.values.chunks(3) {
        // Every Elf carries a badge that identifies their group.
        let a: HashSet<char> = group[0].chars().collect();
        let b: HashSet<char> = group[1].chars().collect();
        let c: HashSet<char> = group[2].chars().collect();

        // For efficiency, within each group of three Elves,
        // the badge is the only item type carried by all three Elves.
        badges.extend(&(&a & &b) & &c)
    }

    // What is the sum of the priorities of those item types?
    badges.iter().map(|&c| priority(c)).sum::<u32>()
}

/// To help prioritize item rearrangement,
/// every item type can be converted to a priority.
fn priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        // Lowercase item types a through z have priorities 1 through 26.
        item as u32 - 'a' as u32 + 1
    } else {
        // Uppercase item types A through Z have priorities 27 through 52.
        item as u32 - 'A' as u32 + 27
    }
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
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 70);
    }
}
