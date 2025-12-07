//! Advent of Code 2025: Day 5
//! <https://adventofcode.com/2025/day/5>

use std::ops::Range;
use std::{fs, io};
use std::path::Path;

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
    let mut count = 0;

    for id in input.available.iter().copied() {
        for range in &input.fresh {
            if range.contains(&id) {
                count += 1;
                break;
            }

            if range.start > id {
                break;
            }
        }
    }

    count
}

fn part2(input: &Input) -> usize {
    let mut total = 0;

    let mut cur = 0;
    for range in &input.fresh {
        let valid = range.start.max(cur)..range.end;
        let count = valid.end.saturating_sub(valid.start) as usize;

        total += count;
        cur = cur.max(range.end);
    }

    total
}

type IngredientID = u64;

#[derive(Debug, Clone)]
struct Input {
    fresh: Vec<Range<IngredientID>>,
    available: Vec<IngredientID>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let (fresh, available) = input.trim().split_once("\n\n").unwrap();

        let mut fresh: Vec<Range<u64>> = fresh.lines().map(|line| {
            let (start, end) = line.split_once('-').expect("range should be hyphen delimited");
            let (start, end) = (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap());

            start..(end + 1)
        }).collect();
        fresh.sort_by_key(|range| range.start);

        let available = available.lines().map(|line| {
            line.parse().expect("ingredient ID should be valid integer")
        }).collect();

        Ok(Self { fresh, available })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 607);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 14);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 342433357244012);
    }
}
