//! Advent of Code 2024: Day 8
//! https://adventofcode.com/2024/day/8

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use std::path::Path;

use lib::vector::Vector;

type Vec2 = Vector<i32, 2>;

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
    let mut freqs: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for (pos, c) in input.antennas.iter() {
        freqs.entry(*c).or_default().push(*pos);
    }

    let mut antinodes = BTreeSet::new();
    for (_, positions) in freqs.into_iter() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }

                let p1 = positions[i];
                let p2 = positions[j];

                let d = p2 - p1;

                let a1 = p2 + d;
                if in_bounds(&input.bounds, a1) {
                    antinodes.insert(a1);
                }

                let a2 = p1 - d;
                if in_bounds(&input.bounds, a2) {
                    antinodes.insert(a2);
                }
            }
        }

    }

    antinodes.len()
}

fn part2(input: &Input) -> usize {
    let mut freqs: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for (pos, c) in input.antennas.iter() {
        freqs.entry(*c).or_default().push(*pos);
    }

    let mut antinodes = BTreeSet::new();
    for (_, positions) in freqs.into_iter() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }

                let p1 = positions[i];
                let p2 = positions[j];

                let d = p2 - p1;

                let mut p = p2;
                loop {
                    if !in_bounds(&input.bounds, p) {
                        break;
                    }

                    antinodes.insert(p);
                    p += d;
                }

                let mut p = p1;
                loop {
                    if !in_bounds(&input.bounds, p) {
                        break;
                    }

                    antinodes.insert(p);
                    p -= d;
                }
            }
        }
    }

    antinodes.len()
}

fn in_bounds(bounds: &(Range<i32>, Range<i32>), pos: Vec2) -> bool {
    bounds.0.contains(&pos[0]) && bounds.1.contains(&pos[1])
}

#[derive(Debug, Clone)]
struct Input {
    bounds: (Range<i32>, Range<i32>),
    antennas: BTreeMap<Vec2, char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut width = 0;
        let mut height = 0;
        let mut antennas = BTreeMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let pos = Vector::new([x as i32, y as i32]);

                if c != '.' {
                    antennas.insert(pos, c);
                }

                width = x as i32 + 1;
                height = y as i32 + 1;
            }
        }

        let bounds = (0..width, 0..height);

        Ok(Self { bounds, antennas })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 423);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 34);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 1287);
    }
}
