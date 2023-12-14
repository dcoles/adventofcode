//! Advent of Code 2023: Day 14 "Parabolic Reflector Dish"
//! https://adventofcode.com/2023/day/14

use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::{fs, io};
use std::path::Path;

// 1 *BILLION* cycles
const CYCLES: usize = 1_000_000_000;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut map = input.rocks.clone();
    for col in 0..input.width {
        let positions: Vec<_> = map.keys().filter(|p| p.x == col).copied().collect();

        let mut blocked_at = 0;
        for pos in positions {
            match map[&pos] {
                Rock::Round => {
                    let rock = map.remove(&pos).unwrap();
                    map.insert(Pos { x: pos.x, y: blocked_at }, rock);
                    blocked_at += 1;
                },
                Rock::Cube => {
                    blocked_at = pos.y + 1;
                },
            }
        }
    }

    map.into_iter()
    .filter_map(|(p, r)| matches!(r, Rock::Round).then_some(input.height - p.y))
    .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    North,
    West,
    South,
    East,
}

fn part2(input: &Input) -> usize {
    let mut map = input.rocks.clone();
    let mut seen = HashMap::new();
    let mut repeats_at = None;

    for cycle in 1..=CYCLES {
        // Phases: North (0), West (1), South (2), East (3)
        //for phase in 0..4 {
        for phase in [Phase::North, Phase::West, Phase::South, Phase::East] {
            // South and East are moving in the reverse direction
            let reversed = matches!(phase, Phase::South | Phase::East);

            if matches!(phase, Phase::North | Phase::South) {
                for col in 0..input.width {
                    let mut positions: Vec<_> = map.keys().filter(|p| p.x == col).copied().collect();

                    if reversed {
                        positions.reverse();
                    }

                    let mut blocked_at = 0;

                    for pos in positions {
                        match map[&pos] {
                            Rock::Round => {
                                let rock = map.remove(&pos).unwrap();

                                let y = if reversed { input.height - blocked_at -  1} else { blocked_at };

                                map.insert(Pos { x: pos.x, y }, rock);
                                blocked_at += 1;
                            },
                            Rock::Cube => {
                                blocked_at = if reversed { input.height.saturating_sub(pos.y) } else { pos.y + 1 };
                            },
                        }
                    }
                }
            } else {
                // West / East
                for row in 0..input.height {
                    let mut positions: Vec<_> = map.keys().filter(|p| p.y == row).copied().collect();

                    if reversed {
                        positions.reverse();
                    }

                    let mut blocked_at = 0;

                    for pos in positions {
                        match map[&pos] {
                            Rock::Round => {
                                let rock = map.remove(&pos).unwrap();

                                let x = if reversed { input.width - blocked_at -  1} else { blocked_at };

                                map.insert(Pos { x, y: pos.y }, rock);
                                blocked_at += 1;
                            },
                            Rock::Cube => {
                                blocked_at = if reversed { input.width.checked_sub(pos.x).unwrap() } else { pos.x + 1 };
                            },
                        }
                    }
                }
            }
        }

        // Find where the cycle repeats

        let hash = map.iter()
        .filter_map(|(&p, &r)| matches!(r, Rock::Round).then_some(p))
        .fold(DefaultHasher::new(), |mut h, p| { p.hash(&mut h); h })
        .finish();

        if let Some(&n) = seen.get(&hash) {
            if let Some((first_n, m)) = repeats_at {
                if (cycle - first_n) % m == (CYCLES - first_n) % m {
                    break;
                }
            } else {
                // n modulo m
                repeats_at = Some((n, cycle - n));
            }
        }

        seen.insert(hash, cycle);
    }

    map.into_iter()
    .filter_map(|(p, r)| matches!(r, Rock::Round).then_some(input.height - p.y))
    .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Pos { x: usize, y: usize }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Round,
    Cube,
}

#[derive(Debug, Clone)]
struct Input {
    rocks: BTreeMap<Pos, Rock>,
    width: usize,
    height: usize,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut width = 0;
        let mut height = 0;
        let mut rocks = BTreeMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Pos { x, y };
                match c {
                    '#' => {
                        rocks.insert(pos, Rock::Cube);
                    },
                    'O' => {
                        rocks.insert(pos, Rock::Round);
                    },
                    _ => (),
                };

                width = width.max(x + 1);
            }

            height = height.max(y + 1);
        }

        Ok(Self { rocks, width, height })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 136);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 113424);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 64);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 96003);
    }
}
