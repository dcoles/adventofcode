/// Advent of Code 2021: Day 3
/// https://adventofcode.com/2021/day/3

use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let input = read_input_from_file("day03/input.txt")?;
    println!("Input: {:?}", input);

    Ok(())
}

fn read_input_from_file(path: impl AsRef<Path>) -> anyhow::Result<String> {
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
