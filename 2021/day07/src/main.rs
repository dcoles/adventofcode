/// Advent of Code 2021: Day 7
/// https://adventofcode.com/2021/day/7

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = read_input_from_file("day07/input.txt").expect("failed to read input");
    println!("Input: {:?}", input);
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<String> {
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
