//! Advent of Code 2024: Day 18
//! https://adventofcode.com/2024/day/18

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;
use std::path::Path;
use lib::vector::Vector;

type Vec2 = Vector<i32, 2>;

const BOUNDS: [Range<i32>; 2] = [0..71, 0..71];
const TEST_BOUNDS: [Range<i32>; 2] = [0..7, 0..7];

const UP: Vec2 = Vec2::new([0, -1]);
const DOWN: Vec2 = Vec2::new([0, 1]);
const LEFT: Vec2 = Vec2::new([-1, 0]);
const RIGHT: Vec2 = Vec2::new([1, 0]);

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let corrupted = input.values.iter().copied().take(1024).collect();

    find_best_path(&BOUNDS, &corrupted).expect("there should be a solution")
}

fn find_best_path(bounds: &[Range<i32>; 2], corrupted: &BTreeSet<Vec2>) -> Option<usize> {
    let start = Vec2::new([bounds[0].start, bounds[1].start]);
    let end = Vec2::new([bounds[0].end - 1, bounds[1].end - 1]);

    let mut edge = vec![start];
    let mut best: BTreeMap<Vec2, usize> = [(start, 0)].into_iter().collect();

    while let Some(pos) = edge.pop() {
        let cur_distance = best[&pos];

        if pos == end {
            return Some(cur_distance);
        }

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let new_pos = pos + dir;
            if !bounds[0].contains(&new_pos[0]) || !bounds[1].contains(&new_pos[1]) {
                continue;
            }

            if corrupted.contains(&new_pos) {
                continue;
            }

            let new_distance = cur_distance + 1;
            if let Some(&best_distance) = best.get(&new_pos) {
                if best_distance <= new_distance {
                    continue;
                }
            }

            edge.push(new_pos);
            best.insert(new_pos, new_distance);
        }

        edge.sort_by_key(|&p| usize::MAX - (best[&p] + distance(end, p)));
    }

    None
}

fn distance(p2: Vec2, p1: Vec2) -> usize {
    let delta = p2 - p1;

    (delta[0].abs() + delta[1].abs()) as usize
}

fn part2(input: &Input) -> Vec2 {
    find_first_cutoff(&BOUNDS, &input.values)
}

fn find_first_cutoff(bounds: &[Range<i32>; 2], corrupted: &[Vec2]) -> Vec2 {
    let mut low = 0;
    let mut high = corrupted.len();

    // Perform binary search
    while high - low > 1 {
        let n = low + (high - low) / 2;
        let corrupted: BTreeSet<Vec2> = corrupted.iter().copied().take(n).collect();

        if find_best_path(bounds, &corrupted).is_none() {
            high = n;
        } else {
            low = n;
        }
    }

    corrupted[low]
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Vec2>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();

            Vec2::new([x.parse().unwrap(), y.parse().unwrap()])
        }).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        let corrupted = input.values.iter().copied().take(12).collect();

        assert_eq!(find_best_path(&TEST_BOUNDS, &corrupted), Some(22));
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 408);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(find_first_cutoff(&TEST_BOUNDS, &input.values), Vec2::new([6, 1]));
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(find_first_cutoff(&BOUNDS, &input.values), Vec2::new([45, 16]));
    }
}
