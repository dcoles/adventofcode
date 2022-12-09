//! Advent of Code 2022: Day 9
//! https://adventofcode.com/2022/day/9

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

const ORIGIN: (i32, i32) = (0, 0);

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    simulate(input, 2)
}

fn part2(input: &Input) -> usize {
    simulate(input, 10)
}

/// Simulate a rope with `n` knots (min: 2).
fn simulate(input: &Input, n: usize) -> usize {
    assert!(n > 1);

    let mut knots = vec![ORIGIN; n];
    let mut visited: HashSet<_> = [ORIGIN].into_iter().collect();

    // Simulate your complete series of motions.
    for motion in &input.values {
        for _ in 0..motion.steps {
            // Move the head
            knots[0] = match motion.direction {
                'U' => (knots[0].0, knots[0].1 + 1),
                'D' => (knots[0].0, knots[0].1 - 1),
                'L' => (knots[0].0 - 1, knots[0].1),
                'R' => (knots[0].0 + 1, knots[0].1),
                d => panic!("Unknown direction: {}", d),
            };

            // Update the remainder of the knots
            for i in 1..n {
                let dx = knots[i - 1].0 - knots[i].0;
                let dy = knots[i - 1].1 - knots[i].1;

                // The this knot and the one pulling it must be touching
                if dx.abs() > 1 || dy.abs() > 1 {
                    // If not, move a maximum of 1 unit in each axis
                    knots[i] = (knots[i].0 + dx.clamp(-1, 1), knots[i].1 + dy.clamp(-1, 1));
                }
            }

            // Keep track of where the tail has been
            let tail = knots[n - 1];
            visited.insert(tail);
        }
    }

    // How many positions does the tail of the rope visit at least once?
    visited.len()
}


#[derive(Debug, Clone)]
struct Input {
    values: Vec<Motion>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            let (direction, steps) = line.split_once(" ").unwrap();
            let direction = direction.chars().next().unwrap();
            let steps: usize = steps.parse().unwrap();

            values.push(Motion { direction, steps });
        }

        Ok(Input { values })
    }
}

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
struct Motion {
    direction: char,
    steps: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 1);
    }

    #[test]
    fn test_part2_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 36);
    }
}
