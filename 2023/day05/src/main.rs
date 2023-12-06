//! Advent of Code 2023: Day 5
//! https://adventofcode.com/2023/day/5

use std::collections::BTreeMap;
use std::fs;
use std::io;
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

fn part1(input: &Input) -> u64 {
    const CONV: [&str; 8] = ["seed", "soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];

    let mut positions = Vec::new();
    for &seed in &input.seeds {
        let mut value = seed;
        for i in 0..(CONV.len() - 1) {
            let from_to = (CONV[i].to_owned(), CONV[i + 1].to_owned());

            for &(source_start, dest_start, len) in &input.mappings[&from_to] {
                if value >= source_start && value < (source_start + len) {
                    value = value + dest_start - source_start;
                    break;
                }
            }
        }

        positions.push(value);
    }

    positions.into_iter().min().unwrap()
}

fn part2(input: &Input) -> u64 {

    let mut ranges = Vec::new();
    for i in 0..input.seeds.len() / 2 {
        let start = input.seeds[2 * i];
        let len = input.seeds[2 * i + 1];

        ranges.push((start, len));
    }

    for i in 0..(CONV.len() - 1) {
        let from_to = (CONV[i].to_owned(), CONV[i + 1].to_owned());

        let mut next_ranges = Vec::new();
        for range in ranges {
            let mut start = range.0;
            let mut len = range.1;

            for &(source_start, dest_start, range_len) in &input.mappings[&from_to] {
                if start < source_start {
                    let temp_len = len.min(source_start - start);
                    if temp_len > 0 {
                        next_ranges.push((start, temp_len));
                    }

                    start += temp_len;
                    len -= temp_len;
                }

                let source_end = source_start + range_len;
                if start >= source_start && start < source_end {
                    let temp_len = len.min(source_end - start);
                    next_ranges.push((start + dest_start - source_start, temp_len));
                    start += temp_len;
                    len -= temp_len;
                }
            }

            if len > 0 {
                next_ranges.push((start, len));
            }
        }

        ranges = next_ranges;
    }

    ranges.into_iter().map(|(a, _)| a).min().unwrap()
}

// (source, destination) => (source_range_start, destination_range_start, length)
type Mapping = BTreeMap<(String, String), Vec<(u64, u64, u64)>>;
#[derive(Debug, Clone)]

struct Input {
    seeds: Vec<u64>,
    mappings: Mapping,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut block_iter = input.split("\n\n");
        let seeds: Vec<u64> = block_iter.next().unwrap().split_ascii_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();

        let mut mappings: Mapping = BTreeMap::new();
        for block in block_iter {
            let mut line_iter = block.lines();
            let (source, destination) = line_iter.next().unwrap().split_ascii_whitespace().next().unwrap().split_once("-to-").unwrap();
            println!("{source} {destination}");
            for line in line_iter {
                let values: Vec<u64> = line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
                let destination_start = values[0];
                let source_start = values[1];
                let length = values[2];

                mappings.entry((source.to_owned(), destination.to_owned())).or_default().push((source_start, destination_start, length));
            }
        }

        for mapping in mappings.values_mut() {
            mapping.sort();
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
