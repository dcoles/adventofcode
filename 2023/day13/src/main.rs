//! Advent of Code 2023: Day 13 "Point of Incidence"
//! https://adventofcode.com/2023/day/13

use std::collections::BTreeSet;
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
    let mut result = 0;

    for map in &input.values {
        // Check for mirroring across the Y-axis (left/right)
        'outer: for x in 1..map.width {
            for (x1, x2) in ((0..x).rev()).zip(x..map.width) {
                for y in 0..map.height {
                    if map.tiles.contains(&Pos { x: x1, y }) != map.tiles.contains(&Pos { x: x2, y }) {
                        continue 'outer;
                    }
                }
            }
            // Mirrored at (x, x + 1)
            result += x;
        }

        // Check for mirroring across the X-axis (top/bottom)
        'outer: for y in 1..map.height {
            for (y1, y2) in (y..map.height).zip((0..y).rev()) {
                for x in 0..map.width {
                    if map.tiles.contains(&Pos { x, y: y1 }) != map.tiles.contains(&Pos { x, y: y2 }) {
                        continue 'outer;
                    }
                }
            }
            // Mirrored at (y, y + 1)
            result += y * 100;
        }
    }

    result
}

fn part2(input: &Input) -> usize {
    let mut result = 0;

    for map in &input.values {
        // Check for mirroring across the Y-axis (left/right)
        'outer: for x in 1..map.width {
            let mut mismatches = 0;
            for (x1, x2) in ((0..x).rev()).zip(x..map.width) {
                for y in 0..map.height {
                    if map.tiles.contains(&Pos { x: x1, y }) != map.tiles.contains(&Pos { x: x2, y }) {
                        mismatches += 1;
                        if mismatches > 1 {
                            continue 'outer;
                        }
                    }
                }
            }

            if mismatches == 1 {
                // Mirrored at (x, x + 1)
                result += x;
            }
        }

        // Check for mirroring across the X-axis (top/bottom)
        'outer: for y in 1..map.height {
            let mut mismatches = 0;
            for (y1, y2) in (y..map.height).zip((0..y).rev()) {
                for x in 0..map.width {
                    if map.tiles.contains(&Pos { x, y: y1 }) != map.tiles.contains(&Pos { x, y: y2 }) {
                        mismatches += 1;
                        if mismatches > 1 {
                            continue 'outer;
                        }
                    }
                }
            }

            if mismatches == 1 {
                // Mirrored at (y, y + 1)
                result += y * 100;
            }
        }
    }

    result
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Pos { x: usize, y: usize }

#[derive(Debug, Clone, Default)]
struct Map {
    tiles: BTreeSet<Pos>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Map>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = vec![];
        for entry in input.split("\n\n") {
            let mut map = Map::default();
            for (y, line) in entry.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if c == '#' {
                        map.tiles.insert(Pos { x: x as usize, y: y as usize });
                    }
                    map.width = x + 1;
                }
                map.height = y + 1;
            }
            values.push(map);
        }

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 405);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 33520);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 400);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 34824);
    }
}
