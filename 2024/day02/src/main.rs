//! Advent of Code 2024: Day 2
//! https://adventofcode.com/2024/day/2

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

fn part1(input: &Input) -> usize {
    input.values.iter().filter(|r| is_safe(&r)).count()
}

/// A report only counts as safe if both of the following are true:
/// - The levels are either all increasing or all decreasing.
/// - Any two adjacent levels differ by at least one and at most three.
fn is_safe(report: &[usize]) -> bool {
    let mut deltas = vec![];
    for pairs in report.windows(2) {
        let delta = (pairs[1] as i32) - (pairs[0] as i32);

        // delta must be at least 1 and no more than 3
        if !(1..=3).contains(&delta.abs()) {
            return false;
        }

        deltas.push(delta);
    }

    // Check if all increasing or all decreasing
    deltas.iter().all(|d| d.signum() != -1) || deltas.iter().all(|d| d.signum() != 1)
}

fn part2(input: &Input) -> usize {
    input.values.iter()
        .filter(|report| {
            // See if the report is safe when ignoring any one value
            (0..report.len()).any(|m| {
                let filtered_report: Vec<_> = report.iter()
                    .copied()
                    .enumerate()
                    .filter_map(|(n, val)| (n != m).then_some(val))
                    .collect();

                is_safe(&filtered_report)
            })
        })
        .count()
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Vec<usize>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            line.split_whitespace().map(|s| s.parse().unwrap()).collect()
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

        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 242);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 311);
    }
}
