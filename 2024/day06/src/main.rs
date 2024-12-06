//! Advent of Code 2024: Day 6
//! https://adventofcode.com/2024/day/6

use std::{fs, io};
use std::collections::BTreeSet;
use std::ops::Range;
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

type Vec2 = (i32, i32);
type Bounds = (Range<i32>, Range<i32>);

/// Add two vectors.
fn add(a: Vec2, b: Vec2) -> Vec2 {
    (a.0 + b.0, a.1 + b.1)
}

/// Check if position is within bounds.
fn in_bounds(pos: Vec2, bounds: &Bounds) -> bool {
    bounds.0.contains(&pos.0) && bounds.1.contains(&pos.1)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Direction turned 90° clockwise.
    fn turn(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Vector corresponding to this direction.
    fn vector(self) -> Vec2 {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

fn part1(input: &Input) -> usize {
    let path = find_path(input.guard, &input.obstructions, &input.bounds).unwrap();
    let unique: BTreeSet<Vec2> = path.into_iter().collect();

    unique.len()
}

/// Find the path the guard follows.
/// Returns `None` if an infinite loop.
fn find_path(start: Vec2, obstructions: &BTreeSet<Vec2>, bounds: &Bounds) -> Option<Vec<Vec2>> {
    let mut dir = Direction::North;
    let mut pos = start;

    let mut path = vec![pos];
    let mut visited: BTreeSet<(Vec2, Direction)> = [(pos, dir)].into_iter().collect();

    loop {
        let mut n_turns = 0;
        while n_turns < 4 && obstructions.contains(&add(pos, dir.vector())) {
            dir = dir.turn();
            n_turns += 1;
        }

        if n_turns == 4 {
            // Infinite loop
            return None;
        }

        pos = add(pos, dir.vector());

        if !in_bounds(pos, bounds) {
            // Guard walked off the map
            break;
        }

        if visited.contains(&(pos, dir)) {
            // Infinite loop
            return None;
        }

        path.push(pos);
        visited.insert((pos, dir));
    }

    Some(path)
}

fn part2(input: &Input) -> usize {
    let path = find_path(input.guard, &input.obstructions, &input.bounds).unwrap();
    let unique: BTreeSet<Vec2> = path.into_iter().collect();

    let mut loop_count = 0;

    for (x, y) in unique {
        if (input.obstructions.contains(&(x, y))) || ((x, y) == input.guard) {
            // Invalid position
            continue;
        }

        let obstructions = input.obstructions.iter().copied()
            .chain([(x, y)])
            .collect();

        if find_path(input.guard, &obstructions, &input.bounds).is_none() {
            loop_count += 1;
        }
    }

    loop_count
}

#[derive(Debug, Clone)]
struct Input {
    bounds: Bounds,
    obstructions: BTreeSet<Vec2>,
    guard: Vec2,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut width = 0;
        let mut height = 0;
        let mut obstructions = BTreeSet::default();
        let mut guard = (0, 0);

        for (y, line) in input.lines().enumerate() {
            height = y as i32 + 1;
            for (x, c) in line.trim().chars().enumerate() {
                let pos = (x as i32, y as i32);
                match c {
                    '#' => { obstructions.insert(pos); },
                    '^' => { guard = pos; },
                    _ => (),
                }
                width = x as i32 + 1;
            }
        }

        let bounds = (0..width, 0..height);
        Ok(Self { bounds, obstructions, guard })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 41);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 4663);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 1530);
    }
}
