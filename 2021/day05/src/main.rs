/// Advent of Code 2021: Day 5
/// https://adventofcode.com/2021/day/5

use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let input = read_input_from_file("day05/input.txt")?;
    println!("Input: {:?}", input);

    Ok(())
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
