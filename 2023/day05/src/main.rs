//! Advent of Code 2023: Day 5
//! https://adventofcode.com/2023/day/5

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::ops::Range;
use std::path::Path;

// Mappings
const CONV: [&str; 8] = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> i64 {
    let mut positions = Vec::new();

    for &seed in &input.seeds {
        let mut value = seed;
        for window in CONV.windows(2) {
            let from_to = (window[0].to_owned(), window[1].to_owned());

            for (source_range, dest_range) in &input.mappings[&from_to] {
                if value >= source_range.start && value < source_range.end {
                    value = value + (dest_range.start - source_range.start);
                    println!("{value}");
                    break;
                }
            }
        }

        positions.push(value);
    }

    // Find the minimum
    positions.into_iter().min().unwrap()
}

fn part2(input: &Input) -> i64 {
    // Initial seed ranges
    let mut seed_ranges = Vec::new();
    for i in 0..input.seeds.len() / 2 {
        let start = input.seeds[2 * i];
        let len = input.seeds[2 * i + 1];

        seed_ranges.push(start..(start + len));
    }

    let mut ranges = seed_ranges.clone();

    // Apply the mappings
    for window in CONV.windows(2) {
        let (conv_from, conv_to) = (window[0].to_owned(), window[1].to_owned());
        let mappings = &input.mappings[&(conv_from, conv_to)];

        ranges = ranges.into_iter().flat_map(|range| {
            let mut new_ranges = vec![];

            let mut start = range.start;
            for (source_range, dest_range) in mappings {
                let offset = dest_range.start - source_range.start;

                // Range before mapping (1:1)
                let mid = range.end.min(source_range.start);
                if (mid - start) > 0 {
                    new_ranges.push(start..mid);
                    start = mid;
                }

                // Overlap
                let end = range.end.min(source_range.end);
                if (end - start) > 0 {
                    new_ranges.push((start + offset)..(end + offset));
                    start = end;
                }

                // Range after mapping handled as range before next mapping
            }

            // Any remainder after last mapping
            if start < range.end {
                new_ranges.push(start..range.end);
            }

            new_ranges
        }).collect();
    }

    ranges.into_iter().map(|r| r.start).min().unwrap()
}

// (source, destination) => (source_range_start, destination_range_start, length)
type Mapping = BTreeMap<(String, String), Vec<(Range<i64>, Range<i64>)>>;
#[derive(Debug, Clone)]

struct Input {
    seeds: Vec<i64>,
    mappings: Mapping,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut block_iter = input.split("\n\n");
        let seeds: Vec<i64> = block_iter.next().unwrap().split_ascii_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

        let mut mappings: Mapping = BTreeMap::new();
        for block in block_iter {
            let mut line_iter = block.lines();
            let (source, destination) = line_iter.next().unwrap().split_ascii_whitespace().next().unwrap().split_once("-to-").unwrap();
            for line in line_iter {
                let values: Vec<i64> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();

                let source_range = values[1]..(values[1] + values[2]);
                let destination_range = values[0]..(values[0] + values[2]);

                mappings.entry((source.to_owned(), destination.to_owned())).or_default().push((source_range, destination_range));
            }
        }

        for mapping in mappings.values_mut() {
            mapping.sort_by_key(|(m, _)| m.start);
        }

        Ok(Input { seeds, mappings })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 46);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 214922730);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 148041808);
    }
}
