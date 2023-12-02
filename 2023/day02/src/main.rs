//! Advent of Code 2023: Day 2
//! https://adventofcode.com/2023/day/2

use core::panic;
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

fn part1(input: &Input) -> usize {
    let limits = (12, 13, 14);

    let mut sum = 0;
    for game in &input.values {
        if game.samples.iter().any(|&s| s.0 > limits.0 || s.1 > limits.1 || s.2 > limits.2) {
            // This game is impossible
            continue;
        }

        sum += game.id;
    }

    sum
}

fn part2(input: &Input) -> usize {
    let mut sum = 0;
    for game in &input.values {
        let minimums = game.samples.iter().fold((0, 0, 0), |a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)));

        let power = minimums.0 * minimums.1 * minimums.2;
        sum += power;
    }

    sum
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    samples: Vec<(usize, usize, usize)> // R, G, B
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Game>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            let (game, samples) = line.split_once(":").unwrap();
            let (_, id) = game.split_once(" ").unwrap();
            let id: usize = id.parse().unwrap();

            let mut gm = Game { id, samples: Vec::new() };
            for sample in samples.trim().split(";") {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                for cube in sample.trim().split(",") {
                    let (n, color) = cube.trim().split_once(" ").unwrap();
                    let n: usize = n.parse().unwrap();
                    match color {
                        "red" => r = n,
                        "green" => g = n,
                        "blue" => b = n,
                        _ => panic!("unknown color {}", color),
                    }
                }
                gm.samples.push((r, g, b));
            }

            values.push(gm);
        }

        Ok(Input { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 8);
    }

    #[test]
    fn test_part2_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 2286);
    }
}
