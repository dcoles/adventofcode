//! Advent of Code 2024: Day 12
//! https://adventofcode.com/2024/day/12

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use std::path::Path;
use lib::vector::Vector;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example2.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

const UP: Vec2 = Vec2::new([0, -1]);
const DOWN: Vec2 = Vec2::new([0, 1]);
const LEFT: Vec2 = Vec2::new([-1, 0]);
const RIGHT: Vec2 = Vec2::new([1, 0]);

fn part1(input: &Input) -> usize {
    let mut regions = Vec::new();

    let mut visitied = BTreeSet::new();
    for y in input.bounds.1.clone() {
        for x in input.bounds.0.clone() {
            let pos = Vec2::new([x, y]);
            if visitied.contains(&pos) {
                continue;
            }

            let plant = input.map[&pos];
            let mut perimenter = 0;
            let mut area = 0;

            let mut edge = vec![pos];
            while let Some(pos) = edge.pop() {
                if visitied.contains(&pos) {
                    continue;
                }

                visitied.insert(pos);

                let plant = input.map[&pos];
                area += 1;

                for adj in [UP, DOWN, LEFT, RIGHT].into_iter().map(|d| pos + d) {
                    if input.map.get(&adj) != Some(&plant) {
                        perimenter += 1;
                    }

                    if !input.map.contains_key(&adj) {
                        continue;
                    }

                    if visitied.contains(&adj) {
                        continue;
                    }

                    if input.map[&adj] == plant {
                        edge.push(adj);
                    }
                }
            }

            regions.push((plant, area, perimenter));
        }
    }

    regions.into_iter()
        .map(|(_, a, p)| a * p)
        .sum()
}

fn part2(input: &Input) -> usize {
    let mut regions = Vec::new();

    let mut visitied = BTreeSet::new();
    for y in input.bounds.1.clone() {
        for x in input.bounds.0.clone() {
            let pos = Vec2::new([x, y]);
            if visitied.contains(&pos) {
                continue;
            }

            let plant = input.map[&pos];
            let mut area = 0;
            let mut edges = 0;

            let mut edge = vec![pos];
            while let Some(pos) = edge.pop() {
                if visitied.contains(&pos) {
                    continue;
                }

                visitied.insert(pos);

                let plant = input.map[&pos];
                area += 1;

                for adj in [UP, DOWN, LEFT, RIGHT].into_iter().map(|d| pos + d) {
                    if !input.map.contains_key(&adj) {
                        continue;
                    }

                    if visitied.contains(&adj) {
                        continue;
                    }

                    if input.map[&adj] == plant {
                        edge.push(adj);
                    }
                }

                // Look for corners
                for d1 in [UP, DOWN] {
                    for d2 in [LEFT, RIGHT] {
                        let c1 = input.map.get(&(pos + d1)) == Some(&plant);
                        let c2 = input.map.get(&(pos + d2)) == Some(&plant);
                        let c3 = input.map.get(&(pos + d1 + d2)) == Some(&plant);

                        match (c1, c2, c3) {
                            // A A     A X
                            // A X  or X ?
                            (true, true, false) | (false, false, _) => {
                                edges += 1;
                            },
                            _ => (),
                        }
                    }
                }
            }

            regions.push((plant, area, edges));
        }
    }

    regions.into_iter()
        .map(|(_, a, p)| a * p)
        .sum()
}

type Vec2 = Vector<i32, 2>;

#[derive(Debug, Clone)]
struct Input {
    bounds: (Range<i32>, Range<i32>),
    map: BTreeMap<Vec2, char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut map = BTreeMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let pos  = Vec2::new([x as i32, y as i32]);

                map.insert(pos, c);
                width = width.max(x as i32 + 1);
            }
            height = height.max(y as i32 + 1);
        }

        let bounds = (0..width, 0..height);

        Ok(Self { bounds, map })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 140);
    }

    #[test]
    fn test_part1_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 1930);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1371306);
    }

    #[test]
    fn test_part2_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 80);
    }

    #[test]
    fn test_part2_example2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 1206);
    }

    #[test]
    fn test_part2_example3() {
        let input = Input::from_file("example3.txt").unwrap();

        assert_eq!(part2(&input), 436);
    }

    #[test]
    fn test_part2_example4() {
        let input = Input::from_file("example4.txt").unwrap();

        assert_eq!(part2(&input), 236);
    }

    #[test]
    fn test_part2_example5() {
        let input = Input::from_file("example5.txt").unwrap();

        assert_eq!(part2(&input), 368);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        // Not 798452
        assert_eq!(part2(&input), 805880);
    }
}
