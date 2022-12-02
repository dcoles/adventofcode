//! Advent of Code 2022: Day 01
//! https://adventofcode.com/2022/day/01

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file("day01/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    // Find the Elf carrying the most Calories.
    let max = *input.values.iter().max().unwrap();

    // How many total Calories is that Elf carrying?
    *input.values.iter().find(|&&n| n == max).unwrap()
}

fn part2(input: &Input) -> usize {
    // Find the top three Elves carrying the most Calories.
    let mut values = input.values.clone();
    values.sort();

    // How many Calories are those Elves carrying in total?
    values.iter().rev().take(3).sum::<usize>()
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<usize>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();

        for elf in input.split("\n\n") {
            let mut calories = 0;

            for item in elf.split_ascii_whitespace() {
                calories += item.parse::<usize>().unwrap();
            }

            values.push(calories);
        }


        Ok(Input { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").expect("failed to read example1.txt");

        assert_eq!(part1(&input), 24000);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").expect("failed to read example1.txt");

        assert_eq!(part2(&input), 45000);
    }
}
