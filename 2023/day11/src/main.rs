//! Advent of Code 2023: Day 10
//! https://adventofcode.com/2023/day/10

use std::collections::BTreeSet;
use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    calculate(input, 2)
}

fn part2(input: &Input) -> usize {
    calculate(input, 1000000)
}

fn calculate(input: &Input, factor: usize) -> usize {
    let mut x_expand = BTreeSet::new();
    for x in 0..input.width {
        if !input.galaxies.iter().any(|p| p.0 == x) {
            x_expand.insert(x);
        }
    }

    let mut y_expand = BTreeSet::new();
    for y in 0..input.height {
        if !input.galaxies.iter().any(|p| p.1 == y) {
            y_expand.insert(y);
        }
    }

    let mut lengths: Vec<usize> = vec![];
    for i in 0..input.galaxies.len() {
        for j in (i + 1)..input.galaxies.len() {
            let pos1 = input.galaxies[i];
            let pos2 = input.galaxies[j];

            let x_min = pos1.0.min(pos2.0);
            let x_max = pos1.0.max(pos2.0);
            let y_min = pos1.1.min(pos2.1);
            let y_max = pos1.1.max(pos2.1);

            let dx = (x_max - x_min) + (factor - 1) * x_expand.iter().filter(|&&x| x > x_min && x < x_max).count() as usize;
            let dy = (y_max - y_min) + (factor - 1) * y_expand.iter().filter(|&&y| y > y_min && y < y_max).count() as usize;
            lengths.push(dx + dy);
        }
    }

    lengths.into_iter().sum()
}

type Pos = (usize, usize);

#[derive(Debug, Clone)]
struct Input {
    galaxies: Vec<Pos>,
    width: usize,
    height: usize,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut galaxies = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((x, y));
                }
                width = x;
            }
            height = y;
        }

        Ok(Self { galaxies, width, height })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 374);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 9724940);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(calculate(&input, 10), 1030);
        assert_eq!(calculate(&input, 100), 8410);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 569052586852);
    }
}
