//! Advent of Code 2024: Day 4
//! https://adventofcode.com/2024/day/4

use std::{fs, io};
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("Input: {input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut count = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            // Find 'X'
            if input.values.get(&(x, y)) != Some(&'X') {
                continue;
            }

            // See if we can find the remaining 'MAS'
            for (dx, dy) in [(1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)] {
                for (n, c) in ['M', 'A', 'S'].into_iter().enumerate() {
                    let n = n as i32 + 1;
                    if input.values.get(&(x + n * dx, y + n * dy)) != Some(&c) {
                        break;
                    }

                    if c == 'S' {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part2(input: &Input) -> usize {
    let mut count = 0;

    for y in 0..input.height {
        for x in 0..input.width {
            // Find the middle 'A'
            if input.values.get(&(x, y)).copied() != Some('A') {
                continue;
            }

            // Forward slash
            let a = input.values.get(&(x + 1, y + 1)).copied().unwrap_or_default();
            let b = input.values.get(&(x - 1, y - 1)).copied().unwrap_or_default();

            // Backward slash
            let c = input.values.get(&(x - 1, y + 1)).copied().unwrap_or_default();
            let d = input.values.get(&(x + 1, y - 1)).copied().unwrap_or_default();

            // Check if the neighbouring letters are valid
            if match (a, b, c, d) {
                ('M', 'S', 'M', 'S') => true,
                ('M', 'S', 'S', 'M') => true,
                ('S', 'M', 'M', 'S') => true,
                ('S', 'M', 'S', 'M') => true,
                _ => false,
            } {
                count += 1;
            }
        }
    }

    count
}

#[derive(Debug, Clone)]
struct Input {
    width: i32,
    height: i32,
    values: BTreeMap<(i32, i32), char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut width = 0;
        let mut height = 0;
        let mut values = BTreeMap::default();
        for (y, line) in input.lines().enumerate() {
            height = y as i32 + 1;
            for (x, c) in line.trim().chars().enumerate() {
                values.insert((x as i32, y as i32), c);
                width = x as i32 + 1;
            }
        }

        Ok(Self { width, height, values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 18);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 2500);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 9);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 1933);
    }
}
