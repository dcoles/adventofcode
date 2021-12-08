//! Advent of Code 2021: Day 8
//! https://adventofcode.com/2021/day/8

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = read_input_from_file("day08/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[(Vec<String>, Vec<String>)]) -> usize {
    use self::SevenSegment::*;

    input.iter()
        .flat_map(|(_, output)| output)
        .filter(|output| {
            let n = output.len();

            n == One.num_segments() || n == Four.num_segments() || n == Seven.num_segments() || n == Eight.num_segments()
        })
        .count()
}

fn part2(input: &[(Vec<String>, Vec<String>)]) -> u32 {
    use self::SevenSegment::*;

    let mut result = 0;

    for (signal, output) in input {
        // Count character frequencies
        let mut frequency: HashMap<usize, Vec<String>> = HashMap::new();
        for s in signal {
            frequency.entry(s.len()).or_default().push(s.to_string());
        }

        let mut mapping = HashMap::new();

        // Unique characters
        for digit in [One, Four, Seven, Eight] {
            let num_segments = digit.num_segments();

            // NOTE: This assumes that all unique characters are in the signal!
            assert_eq!(frequency[&num_segments].len(), 1);
            mapping.insert(digit, frequency[&num_segments][0].clone());
        }

        // Six must be the only 6-segment number that does not have all of One's segments
        let six = signal.iter().filter(|s| s.len() == Six.num_segments() && !mapping[&One].chars().all(|c| s.contains(c))).next().unwrap();
        mapping.insert(Six, six.clone());

        // Three must be the only 5-segment number that contains all of One's segments
        let three = signal.iter().filter(|s| s.len() == Three.num_segments() && mapping[&One].chars().all(|c| s.contains(c))).next().unwrap();
        mapping.insert(Three, three.clone());

        // Nine must be the only 6-digit number that contains all of Three's segments
        let nine = signal.iter().filter(|s| s.len() == Nine.num_segments() && mapping[&Three].chars().all(|c| s.contains(c))).next().unwrap();
        mapping.insert(Nine, nine.clone());

        // Zero must be the remaining 6-digit number
        let zero = signal.iter().filter(|s| s.len() == Zero.num_segments() && s.as_str() != mapping[&Six] && s.as_str() != mapping[&Nine]).next().unwrap();
        mapping.insert(Zero, zero.clone());

        // Two must be the only 5-digit number with 2 common digits with Four
        let two = signal.iter().filter(|s| s.len() == Two.num_segments() && mapping[&Four].chars().filter(|&c| s.contains(c)).count() == 2).next().unwrap();
        mapping.insert(Two, two.clone());

        // Five must be the remaining 5-digit number
        let five = signal.iter().filter(|s| s.len() == Five.num_segments() && s.as_str() != mapping[&Two] && s.as_str() != mapping[&Three]).next().unwrap();
        mapping.insert(Five, five.clone());

        let reverse_mapping: HashMap<_, _> = mapping.into_iter().map(|(k, v)| (v, k)).collect();
        let n = output.iter().fold(0, |acc, o| 10 * acc + reverse_mapping[o] as u32);

        result += n;
    }

    result
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SevenSegment {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl SevenSegment {
    fn num_segments(&self) -> usize {
        match self {
            SevenSegment::Zero => 6,
            SevenSegment::One => 2,
            SevenSegment::Two => 5,
            SevenSegment::Three => 5,
            SevenSegment::Four => 4,
            SevenSegment::Five => 5,
            SevenSegment::Six => 6,
            SevenSegment::Seven => 3,
            SevenSegment::Eight => 7,
            SevenSegment::Nine => 6,
        }
    }
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Vec<(Vec<String>, Vec<String>)>> {
    let input = fs::read_to_string(path)?;

    Ok(
        input.lines().map(|line| {
            let (signal, output) = line.split_once(" | ").unwrap();
            let signal: Vec<String> = signal.split_whitespace().map(sort_characters).collect();
            let output: Vec<String> = output.split_whitespace().map(sort_characters).collect();

            (signal, output)
        }).collect()
    )
}

fn sort_characters(s: &str) -> String {
    let mut chars: Vec<_> = s.chars().collect();
    chars.sort();

    chars.into_iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input_from_file("example2.txt").expect("failed to read input");

        assert_eq!(part1(&input), 26);
    }

    #[test]
    fn test_part2_example1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 5353);
    }

    #[test]
    fn test_part2_example2() {
        let input = read_input_from_file("example2.txt").expect("failed to read input");

        assert_eq!(part2(&input), 61229);
    }
}
