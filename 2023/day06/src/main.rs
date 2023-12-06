//! Advent of Code 2023: Day 6
//! https://adventofcode.com/2023/day/6

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

// Parse as space-seperated values
fn parse1(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace().skip(1).map(|s| s.parse().unwrap()).collect()
}

fn part1(input: &Input) -> usize {
    let mut line_iter = input.text.lines();

    let times: Vec<_> = parse1(line_iter.next().unwrap());
    let distances: Vec<_> = parse1(line_iter.next().unwrap());

    let mut margin: Vec<usize> = Vec::new();
    for (time, distance_record) in times.into_iter().zip(distances.into_iter()) {
        let n = (0..time).map(|t| t * (time - t)).filter(|&d| d > distance_record).count();
        margin.push(n);
    }

    margin.into_iter().product()
}

// Parse as a single value
fn parse2(s: &str) -> u64 {
    s.replace(" ", "").split_once(':').map(|(_, b)| b.parse().unwrap()).unwrap()
}

fn part2(input: &Input) -> usize {
    let mut line_iter = input.text.lines();
    let time = parse2(line_iter.next().unwrap());
    let distance_record = parse2(line_iter.next().unwrap());

    (0..time).map(|t| t * (time - t)).filter(|&d| d > distance_record).count()
}

#[derive(Debug, Clone)]
struct Input {
    text: String,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let text = fs::read_to_string(path)?;

        Ok(Input { text })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 288);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 393120);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 71503);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 36872656);
    }
}
