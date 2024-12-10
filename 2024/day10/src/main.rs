//! Advent of Code 2024: Day 10
//! https://adventofcode.com/2024/day/10

use std::{fs, io};
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use lib::vector::Vector;

type Vec2 = Vector<i32, 2>;

const UP: Vec2 = Vec2::new([0, -1]);
const DOWN: Vec2 = Vec2::new([0, 1]);
const LEFT: Vec2 = Vec2::new([-1, 0]);
const RIGHT: Vec2 = Vec2::new([1, 0]);

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example5.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let trailheads: BTreeSet<Vec2> = input.heights.iter()
        .filter_map(|(&p, &h)| (h == 0).then_some(p)).collect();

    trailheads.into_iter().map(|head| score(head, &input.heights)).sum()
}

fn score(trailhead: Vec2, heights: &BTreeMap<Vec2, u8>) -> usize {
    let mut score = 0;

    let mut visited: BTreeSet<Vec2> = [trailhead].into_iter().collect();
    let mut edge = vec![trailhead];

    while let Some(pos) = edge.pop() {
        let height = heights[&pos];

        if height == 9 {
            score += 1;
            continue;
        }

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let next_pos = pos + dir;
            if visited.contains(&next_pos) {
                continue;
            }

            if heights.get(&next_pos).copied() == Some(height + 1) {
                visited.insert(next_pos);
                edge.push(next_pos);
            }
        }
    }

    score
}


fn part2(input: &Input) -> usize {
    let trailheads: BTreeSet<Vec2> = input.heights.iter()
        .filter_map(|(&p, &h)| (h == 0).then_some(p)).collect();

    trailheads.into_iter().map(|head| rating(head, &input.heights)).sum()
}

fn rating(trailhead: Vec2, heights: &BTreeMap<Vec2, u8>) -> usize {
    let mut rating = 0;

    let mut hasher = DefaultHasher::new();
    trailhead.hash(&mut hasher);
    let hash = hasher.finish();

    let mut visited: BTreeSet<u64> = [hash].into_iter().collect();
    let mut edge = vec![(trailhead, hasher)];

    while let Some((pos, hasher)) = edge.pop() {
        let height = heights[&pos];

        if height == 9 {
            rating += 1;
            continue;
        }

        for dir in [UP, DOWN, LEFT, RIGHT] {
            let next_pos = pos + dir;

            let mut hasher = hasher.clone();
            next_pos.hash(&mut hasher);
            let hash = hasher.finish();

            if visited.contains(&hash) {
                continue;
            }

            visited.insert(hash);

            if heights.get(&next_pos).copied() == Some(height + 1) {
                edge.push((next_pos, hasher));
            }
        }
    }

    rating
}

#[derive(Debug, Clone)]
struct Input {
    heights: BTreeMap<Vec2, u8>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut heights = BTreeMap::new();

        for (x, line) in input.lines().enumerate() {
            for (y, c) in line.trim().chars().enumerate() {
                if c == '.' {
                    continue;
                }

                let pos = Vec2::new([x as i32, y as i32]);
                let height = c.to_digit(10).unwrap() as u8;

                heights.insert(pos, height);
            }
        }

        Ok(Self { heights })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example5.txt").unwrap();

        assert_eq!(part1(&input), 36);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 776);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example5.txt").unwrap();

        assert_eq!(part2(&input), 81);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 1657);
    }
}
