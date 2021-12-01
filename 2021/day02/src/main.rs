/// Advent of Code 2021: Day 2
/// https://adventofcode.com/2021/day/2

use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let input = read_input_from_file("day02/input.txt")?;
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
    fn test_day1() {
        todo!();
    }

    #[test]
    fn test_day2() {
        todo!();
    }
}
