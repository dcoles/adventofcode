//! Advent of Code 2025: Day 1
//! https://adventofcode.com/2025/day/1

use std::{fs, io};
use std::path::Path;

const N_POSITIONS: i32 = 100;
const START: i32 = 50;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut count = 0;

    let mut position = START;
    for rotation in &input.values {
        position = match rotation {
            &Rotation::L(n) => (position - n).rem_euclid(N_POSITIONS),
            &Rotation::R(n) => (position + n).rem_euclid(N_POSITIONS),
        };

        // Count the number of times we stop on zero
        if position == 0 {
            count += 1;
        }
    }

    count
}

fn part2(input: &Input) -> i32 {
    let mut count = 0;

    let mut position = START;
    for rotation in &input.values {
        match rotation {
            &Rotation::L(n) => {
                // Rust division rounds towards zero, so we need to normalize first
                count += ((N_POSITIONS - position) % N_POSITIONS + n) / N_POSITIONS;
                position = (position - n).rem_euclid(N_POSITIONS);
            },
            &Rotation::R(n) => {
                count += (position + n) / N_POSITIONS;
                position = (position + n).rem_euclid(N_POSITIONS);
            },
        };
    }

    count
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    L(i32),
    R(i32),
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Rotation>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            let direction = &line[0..1];
            let clicks: i32 = line[1..].parse().expect("should be valid number");

            match direction {
                "L" => Rotation::L(clicks),
                "R" => Rotation::R(clicks),
                _ => panic!("unknown direction {direction:?}"),
            }
        }).collect();

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

        assert_eq!(part1(&input), 1172);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 6932);
    }
}
