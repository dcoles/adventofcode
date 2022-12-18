//! Advent of Code 2022: Day 18
//! https://adventofcode.com/2022/day/18

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

type Pos = (isize, isize, isize);
const OFFSETS: [Pos; 6] = [(0, 0, 1), (0, 1, 0), (1, 0, 0), (0, 0, -1), (0, -1, 0), (-1, 0, 0)];

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    surface_area(&input.values)
}

fn part2(input: &Input) -> usize {
    // Upper and lower bounds of cube positions
    let (lower, upper) = bounds(&input.values);

    // Positions known to be inside the lava droplet
    let mut inside = HashSet::new();

    for cube in input.values.iter().cloned() {
        'face: for adj in adjacent(cube).filter(|c| !input.values.contains(c)) {
            let mut visited = HashSet::new();
            let mut edge: Vec<_> = vec![adj];

            while let Some(c) = edge.pop() {
                visited.insert(c);

                for a in adjacent(c).filter(|&c| !input.values.contains(&c)) {
                    if visited.contains(&a) {
                        // We've already visited this position
                        continue;
                    }

                    if inside.contains(&a) {
                        // We've already mapped this space
                        continue 'face;
                    }

                    if a.0 < lower.0 || a.1 < lower.1 || a.2 < lower.2 || a.0 > upper.0 || a.1 > upper.1 || a.2 > upper.2 {
                        // We must be outside
                        continue 'face;
                    }

                    edge.push(a);
                }
            }
            
            inside.extend(visited);
        }
    }

    surface_area(&input.values) - surface_area(&inside)
}

/// Calculate the number of exposed surfaces on a collection of cubes.
fn surface_area(cubes: &HashSet<Pos>) -> usize {
    cubes.iter()
        .copied()
        .map(|cube| adjacent(cube).filter(|&c| !cubes.contains(&c)).count())
        .sum()
}

/// All immediately adjacent coordinates (always 6 `Pos`).
fn adjacent(pos: Pos) -> impl Iterator<Item = Pos> {
    OFFSETS.into_iter().map(move |o| (pos.0 + o.0, pos.1 + o.1, pos.2 + o.2))
}

/// Lower and upper bounds of collection of cubes.
fn bounds(cubes: &HashSet<Pos>) -> (Pos, Pos) {
    let mut lower = (isize::MAX, isize::MAX, isize::MAX);
    let mut upper = (isize::MIN, isize::MIN, isize::MIN);

    for cube in cubes.iter().copied() {
        lower = (cube.0.min(lower.0), cube.1.min(lower.1), cube.2.min(lower.2));
        upper = (cube.0.max(upper.1), cube.1.max(upper.1), cube.2.max(upper.2));
    }

    (lower, upper)
}

#[derive(Debug, Clone)]
struct Input {
    values: HashSet<Pos>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = HashSet::new();
        for line in input.lines() {
            let val: Vec<_> = line.trim().split(',').map(|s| s.parse::<isize>().unwrap()).collect();
            values.insert((val[0], val[1], val[2]));
        }

        Ok(Input { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 64);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 58);
    }
}
