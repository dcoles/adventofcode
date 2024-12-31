//! Advent of Code 2024: Day 25
//! https://adventofcode.com/2024/day/25

use std::{fs, io};
use std::path::Path;
use std::str::FromStr;
use lib::grid::{Grid, Pos};

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &Input) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in input.values.iter() {
        if schematic[Pos::new([0, 0])] == '#' {
            keys.push(schematic.clone());
        } else {
            locks.push(schematic.clone());
        }
    }

    let mut count = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if !lock.positions().any(|p| lock[p] == '#' && key[p] == '#') {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Grid>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let values = input.split("\n\n")
            .map(|chunk| Grid::from_str(chunk.trim()).unwrap())
            .collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 2586);
    }
}
