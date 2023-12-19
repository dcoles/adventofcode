//! Advent of Code 2023: Day 18 "Lavaduct Lagoon"
//! https://adventofcode.com/2023/day/18

use std::collections::HashSet;
use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut edge_tiles: HashSet<(i64, i64)> = [(x, y)].into_iter().collect();

    for (direction, distance, _) in &input.values {
        for _ in 0..*distance {
            match direction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Left => x -= 1,
                Direction::Right => x += 1,
            }

            edge_tiles.insert((x, y));
        }
    }

    let mut flood_tiles = HashSet::new();
    let mut edge = vec![(1, 1)];
    while let Some(pos) = edge.pop() {
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let adj = (pos.0 + dx, pos.1 + dy);
            if edge_tiles.contains(&adj) || flood_tiles.contains(&adj) {
                continue;
            }

            edge.push(adj);
            flood_tiles.insert(adj);
        }
    }

    edge_tiles.len() + flood_tiles.len()
}

fn part2(input: &Input) -> usize {
    0
}

#[derive(Debug, Clone)]
struct Input {
    /// `(direction, distance, color)`
    values: Vec<(Direction, i64, String)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = vec![];
        for line in input.lines() {
            let mut iter = line.split(' ');
            let direction = Direction::from_str(iter.next().unwrap());
            let distance = iter.next().unwrap().parse::<i64>().unwrap();
            let color = iter.next().unwrap()[1..8].to_string();

            values.push((direction, distance, color));
        }

        Ok(Self { values })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Unknown direction: {s:?}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 62573);
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
