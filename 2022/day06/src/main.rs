//! Advent of Code 2022: Day 6
//! https://adventofcode.com/2022/day/6

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input.datastream));

    // Part 2
    println!("Part 2: {}", part2(&input.datastream));
}

fn part1(datastream: &str) -> usize {
    let len = datastream.len();

    for i in 0..len-3 {
        let chars: HashSet<char> = datastream.chars().skip(i).take(4).collect();
        if chars.len() == 4 {
            return i + 4;
        }
    }

    0
}

fn part2(datastream: &str) -> usize {
    let len = datastream.len();

    for i in 0..len-13 {
        let chars: HashSet<char> = datastream.chars().skip(i).take(14).collect();
        if chars.len() == 14 {
            return i + 14;
        }
    }

    0
}

#[derive(Debug, Clone)]
struct Input {
    datastream: String,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let datastream = fs::read_to_string(path)?;

        Ok(Input { datastream })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
