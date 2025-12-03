//! Advent of Code 2025: Day 3
//! <https://adventofcode.com/2025/day/3>

use std::{fs, io};
use std::path::Path;

const PART2_N_BATTERIES: usize = 12;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut sum = 0;

    for bank in &input.values {
        let tens = *bank[..bank.len() - 1].iter().max().unwrap();
        let i = bank.iter().position(|&n| n == tens).unwrap();

        let ones = *bank[(i + 1)..].iter().max().unwrap();

        let jolts = 10 * tens + ones;

        sum += jolts;
    }

    sum
}

fn part2(input: &Input) -> usize {
    let mut sum = 0;

    for bank in &input.values {
        let mut i = 0;
        let mut jolts = 0;

        for n in 0..PART2_N_BATTERIES {
            let window = &bank[i..(bank.len() - ((PART2_N_BATTERIES - 1) - n))];
            let best = *window.iter().max().unwrap();
            i += window.iter().position(|&n| n == best).unwrap() + 1;

            jolts = jolts * 10 + best;
        }

        sum += jolts;
    }

    sum
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Vec<usize>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let values = input.trim().lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect())
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

        assert_eq!(part1(&input), 357);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 17443);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 3121910778619);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 172167155440541);
    }
}
