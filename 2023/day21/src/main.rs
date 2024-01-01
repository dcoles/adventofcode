//! Advent of Code 2023: Day 21 "Step Counter"
//! https://adventofcode.com/2023/day/21

use std::collections::{BTreeSet, VecDeque, HashSet};
use std::{fs, io};
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input, 64));

    // Part 2
    println!("Part 2: {}", part2(&input, 26501365));
}

fn part1(input: &Input, n_steps: usize) -> usize {
    reachable_tiles(input, n_steps).len()
}

fn part2(input: &Input, n_steps: usize) -> usize {
    // Make sure we can reach all tile types with an odd number of steps
    let reachable: HashSet<Pos> = reachable_tiles(input, 2 * 131 + 65).into_iter().filter(|&pos| is_parity(pos, 1)).collect();

    let full_0 = reachable.iter().copied().filter(|&pos| is_field(pos, 0, 0)).count();
    let full_1 = reachable.iter().copied().filter(|&pos| is_field(pos, 0, 1)).count();
    let cardinals = reachable.iter().copied().filter(|&pos| is_field(pos, 0, -2) || is_field(pos, 0, 2) || is_field(pos, -2, 0) || is_field(pos, 2, 0)).count();
    let corners_a = reachable.iter().copied().filter(|&pos| is_field(pos, -1, -2) || is_field(pos, 1, -2) || is_field(pos, -1, 2) || is_field(pos, 1, 2)).count();
    let corners_b = reachable.iter().copied().filter(|&pos| is_field(pos, -1, -1) || is_field(pos, 1, -1) || is_field(pos, -1, 1) || is_field(pos, 1, 1)).count();

    let n = n_steps / input.width as usize;
    let a = 4 * ((n + 1) / 2) * ((n - 1) / 2) + 1;
    let b = 4 * (n / 2) * (n / 2);

    a * full_0 + b * full_1 + cardinals + n * corners_a + (n - 1) * corners_b
}

fn is_field(pos: Pos, x: i64, y: i64) -> bool {
    pos.0.div_euclid(131) == x && pos.1.div_euclid(131) == y
}

fn is_parity(pos: Pos, n: usize) -> bool {
    (pos.0 + pos.1).rem_euclid(2) as usize == n.rem_euclid(2)
}

fn reachable_tiles(input: &Input, steps: usize) -> HashSet<Pos> {
    let mut edge: VecDeque<_> = [(input.start, 0)].into_iter().collect();
    let mut seen: HashSet<Pos> = [input.start].into_iter().collect();
    while let Some((pos, cur_steps)) = edge.pop_front() {
        let new_steps = cur_steps + 1;

        for delta in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let adj = Pos(pos.0 + delta.0, pos.1 + delta.1);
            if input.rocks.contains(&Pos(adj.0.rem_euclid(input.width), adj.1.rem_euclid(input.height))) {
                continue;
            }

            if seen.contains(&adj) {
                continue;
            }
            seen.insert(adj);

            if new_steps < steps {
                edge.push_back((adj, new_steps));
            }
        }
    }

    seen.into_iter().filter(|&pos| is_parity(pos, steps)).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos(i64, i64);

#[derive(Debug, Clone)]
struct Input {
    start: Pos,
    rocks: BTreeSet<Pos>,
    width: i64,
    height: i64,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut start = Pos(0, 0);
        let mut rocks = BTreeSet::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Pos(x as i64, y as i64);

                match c {
                    '#' => { rocks.insert(pos); },
                    'S' => { start = pos; },
                    '.' => (),
                    _ => panic!("unknown tile: {c:?}"),
                }

                width = width.max(x as i64 + 1);
            }
            height = height.max(y as i64 + 1);
        }

        Ok(Self { start, rocks, width, height })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input, 6), 16);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input, 64), 3740);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(reachable_tiles(&input, 6).len(), 16);
        assert_eq!(reachable_tiles(&input, 10).len(), 50);
        assert_eq!(reachable_tiles(&input, 100).len(), 6536);
        assert_eq!(reachable_tiles(&input, 500).len(), 167004);
        assert_eq!(reachable_tiles(&input, 1000).len(), 668697);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        // Some sanity checking
        //assert_eq!(reachable_tiles(&input, 2 * 131 + 65).len(), part2(&input, 2 * 131 + 65));
        //assert_eq!(reachable_tiles(&input, 4 * 131 + 65).len(), part2(&input, 4 * 131 + 65));
        //assert_eq!(reachable_tiles(&input, 8 * 131 + 65).len(), part2(&input, 8 * 131 + 65));

        assert_eq!(part2(&input, 26501365), 620962518745459);
    }
}
