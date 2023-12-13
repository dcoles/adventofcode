//! Advent of Code 2023: Day 12
//! https://adventofcode.com/2023/day/12

use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut combinations = vec![];
    for record in &input.values {
        let unknown_index: Vec<usize> = (0..record.0.len()).filter(|&i| matches!(record.0[i], State::Unknown)).collect();
        let n_combinations: usize = 2 << (unknown_index.len() - 1);
        let mut state = record.0.clone();

        let mut count = 0;
        for n in 0..n_combinations {
            for (i, &x) in unknown_index.iter().enumerate() {
                state[x] = if n >> i & 1 == 1 { State::Damaged } else { State::Operational };
            }

            if is_match(&state, &record.1) {
                count += 1;
            }
        }

        combinations.push(count);
    }

    println!("{:?}", combinations);
    combinations.into_iter().sum()
}

fn is_match(state: &[State], groups: &[usize]) -> bool {
    let mut index = 0;
    let mut run = 0;

    for (i, s) in state.iter().enumerate() {
        let damaged = matches!(s, State::Damaged);
        if damaged {
            run += 1;
        }

        let last = i == (state.len() - 1);
        if (!damaged || last) && run > 0 {
            if index >= groups.len() {
                return false;
            }

            if groups[index] != run {
                return false;
            }

            index += 1;
            run = 0;
        }
    }

    index == groups.len()
}

fn part2(input: &Input) -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Unknown,
    Operational,
    Damaged,
}

impl State {
    fn from_char(c: char) -> Self {
        match c {
            '?' => Self::Unknown,
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => panic!("Unknown state: {c:?}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(Vec<State>, Vec<usize>)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            let a = a.chars().map(State::from_char).collect();
            let b = b.split(',').map(|s| s.parse().unwrap()).collect();

            (a, b)
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

        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 7025);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 0);
    }
}
