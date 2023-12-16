//! Advent of Code 2023: Day 16 "The Floor Will Be Lava"
//! https://adventofcode.com/2023/day/X

use std::collections::{BTreeMap, HashSet};
use std::{fs, io, i64};
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
    simulate(input, Pos(0, 0), Direction::Right)
}

fn part2(input: &Input) -> usize {
    let mut values = vec![];
    for column in 0..input.width {
        values.push(simulate(input, Pos(column, 0), Direction::Down));
        values.push(simulate(input, Pos(column, input.height - 1), Direction::Up));
    }

    for row in 0..input.height {
        values.push(simulate(input, Pos(0, row), Direction::Right));
        values.push(simulate(input, Pos(input.width - 1, row), Direction::Left));
    }

    values.into_iter().max().unwrap()
}

fn simulate(input: &Input, initial_position: Pos, initial_direction: Direction) -> usize {
    let start = (initial_position, initial_direction);
    let mut seen: HashSet<(Pos, Direction)> = [start].into_iter().collect();
    let mut edge: Vec<(Pos, Direction)> = vec![start];
    while let Some((pos, dir)) = edge.pop() {
        let mut new_entries = vec![];
        if let Some(entity) = input.map.get(&pos) {
            match entity {
                Entity::Mirror(Mirror::Forward) => match dir {
                    Direction::Up => new_entries.push((pos.to(Direction::Right), Direction::Right)),
                    Direction::Down => new_entries.push((pos.to(Direction::Left), Direction::Left)),
                    Direction::Left => new_entries.push((pos.to(Direction::Down), Direction::Down)),
                    Direction::Right => new_entries.push((pos.to(Direction::Up), Direction::Up)),
                },
                Entity::Mirror(Mirror::Backward) => match dir {
                    Direction::Up => new_entries.push((pos.to(Direction::Left), Direction::Left)),
                    Direction::Down => new_entries.push((pos.to(Direction::Right), Direction::Right)),
                    Direction::Left => new_entries.push((pos.to(Direction::Up), Direction::Up)),
                    Direction::Right => new_entries.push((pos.to(Direction::Down), Direction::Down)),
                },
                Entity::Splitter(Splitter::Horizontal) => match dir {
                    Direction::Up | Direction::Down => {
                        new_entries.push((pos.to(Direction::Left), Direction::Left));
                        new_entries.push((pos.to(Direction::Right), Direction::Right));
                    },
                    Direction::Left | Direction::Right => {
                        // Continues unaffected
                        new_entries.push((pos.to(dir), dir));
                    },
                },
                Entity::Splitter(Splitter::Vertical) => match dir {
                    Direction::Up | Direction::Down => {
                        // Continues unaffected
                        new_entries.push((pos.to(dir), dir));
                    },
                    Direction::Left | Direction::Right => {
                        new_entries.push((pos.to(Direction::Up), Direction::Up));
                        new_entries.push((pos.to(Direction::Down), Direction::Down));
                    },
                },
            }
        } else {
            // Continues unaffected
            new_entries.push((pos.to(dir), dir));
        }

        let valid = |pos: Pos| (0..input.width).contains(&pos.0) && (0..input.height).contains(&pos.1);

        edge.extend(new_entries.iter().copied().filter(|entry| !seen.contains(entry) && valid(pos)));
        seen.extend(new_entries.into_iter().filter(|&(pos, _)| valid(pos)));
    }

    let energized: HashSet<Pos> = seen.into_iter().map(|(pos, _)| pos).collect();

    energized.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos(i64, i64);

impl Pos {
    fn to(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Pos(self.0, self.1 - 1),
            Direction::Down => Pos(self.0, self.1 + 1),
            Direction::Left => Pos(self.0 - 1, self.1),
            Direction::Right => Pos(self.0 + 1, self.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mirror {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Splitter {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Entity {
    Mirror(Mirror),
    Splitter(Splitter),
}

#[derive(Debug, Clone)]
struct Input {
    map: BTreeMap<Pos, Entity>,
    width: i64,
    height: i64,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let mut map = BTreeMap::new();

        let mut width = 0;
        let mut height = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let entity = match c {
                    '/' => Some(Entity::Mirror(Mirror::Forward)),
                    '\\' => Some(Entity::Mirror(Mirror::Backward)),
                    '|' => Some(Entity::Splitter(Splitter::Vertical)),
                    '-' => Some(Entity::Splitter(Splitter::Horizontal)),
                    '.' => None,
                    _ => panic!("Unrecognized tile: {c:?}"),
                };

                if let Some(entity) = entity {
                    map.insert(Pos(x as i64, y as i64), entity);
                    width = width.max(x as i64 + 1);
                }
            }
            height = height.max(y as i64 + 1);
        }

        Ok(Self { map, width, height })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 6906);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 51);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 7330);
    }
}
