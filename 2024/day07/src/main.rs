//! Advent of Code 2024: Day 7
//! https://adventofcode.com/2024/day/7

use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Op::Add => left + right,
            Op::Mul => left * right,
            Op::Concat => left * right.next_multiple_of(10) + right,
        }
    }
}

fn part1(input: &Input) -> u64 {
    let mut sum = 0;

    for (result, values) in input.values.iter() {
        let result = *result;

        let mut edge = vec![(1, values[0])];
        while let Some((n, x)) = edge.pop() {
            if n == values.len() {
                if x == result {
                    // solved!
                    sum += result;
                    break;
                }
            } else {
                for op in [Op::Add, Op::Mul] {
                    let val = op.apply(x, values[n]);
                    if val <= result {
                        edge.push((n + 1, val));
                    }
                }
            }
        }
    }

    sum
}

fn part2(input: &Input) -> u64 {
    let mut sum = 0;

    for (result, values) in input.values.iter() {
        let result = *result;

        let mut edge = vec![(1, values[0])];
        while let Some((n, x)) = edge.pop() {
            if n == values.len() {
                // solved!
                if x == result {
                    sum += result;
                    break;
                }
            } else {
                for op in [Op::Add, Op::Mul, Op::Concat] {
                    let val = op.apply(x, values[n]);
                    if val <= result {
                        edge.push((n + 1, val));
                    }
                }
            }
        }
    }

    sum
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(u64, Vec<u64>)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            let (left, right) = line.trim().split_once(": ").unwrap();
            let left = left.parse().unwrap();
            let right = right.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();

            (left, right)
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

        assert_eq!(part1(&input), 3749);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 5512534574980);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 11387);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 328790210468594);
    }
}
