//! Advent of Code 2021: Day 13
//! https://adventofcode.com/2021/day/13

use std::fs;
use std::io;
use std::path::Path;

type Input = String;

fn main() {
    let input = read_input_from_file("day13/input.txt").expect("failed to read input");
    println!("Input: {:?}", input);

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    0
}

fn part2(input: &Input) -> usize {
    0
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Input> {
    let input = fs::read_to_string(path)?;

    Ok(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        todo!();
    }

    #[test]
    fn test_part2() {
        todo!();
    }
}
