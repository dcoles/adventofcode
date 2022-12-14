//! Advent of Code 2022: Day 14
//! https://adventofcode.com/2022/day/14

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const SAND: char = 'o';
const ROCK: char = '#';
const SOURCE: (usize, usize) = (500, 0);

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut map = input.map.clone();
    let abyss = map.keys().map(|p| p.1 + 2).max().unwrap();

    for unit in 1.. {
        // Sand starts at the source
        let mut pos = SOURCE;

        loop {
            loop {
                if pos.1 > abyss {
                    // This unit fell into the abyss
                    // Return the last one that came to rest
                    return unit - 1;
                }

                if map.contains_key(&(pos.0, pos.1 + 1)) {
                    // We hit something!
                    break;
                }

                // Keep falling...
                pos = (pos.0, pos.1 + 1);
            }

            // See if we can slide diagonally
            if !map.contains_key(&(pos.0 - 1, pos.1 + 1)) {
                // Diagonal left was empty
                pos = (pos.0 - 1, pos.1 + 1);

                continue;
            } else if !map.contains_key(&(pos.0 + 1, pos.1 + 1)) {
                // Diagonal right was empty
                pos = (pos.0 + 1, pos.1 + 1);

                continue;
            }

            // We must be blocked - this is where we come to rest
            map.insert((pos.0, pos.1), SAND);

            break;
        }
    }

    unreachable!();
}

fn part2(input: &Input) -> usize {
    let mut map = input.map.clone();
    let floor = map.keys().map(|p| p.1 + 2).max().unwrap();

    for unit in 1.. {
        // Sand starts at the source
        let mut pos = SOURCE;

        loop {
            loop {
                if map.contains_key(&(pos.0, pos.1 + 1)) || pos.1 + 1 == floor {
                    // We hit something!
                    break;
                }

                // Keep falling...
                pos = (pos.0, pos.1 + 1);
            }

            // See if we can slide diagonally
            if !map.contains_key(&(pos.0 - 1, pos.1 + 1)) && pos.1 + 1 != floor {
                // Diagonal left was empty
                pos = (pos.0 - 1, pos.1 + 1);

                continue;
            } else if !map.contains_key(&(pos.0 + 1, pos.1 + 1)) && pos.1 + 1 != floor {
                // Diagonal right was empty
                pos = (pos.0 + 1, pos.1 + 1);

                continue;
            }

            // We must be blocked - this is where we come to rest
            map.insert((pos.0, pos.1), SAND);

            if pos == SOURCE {
                // We blocked the source
                return unit
            }

            break;
        }
    }

    unreachable!();
}

#[derive(Debug, Clone)]
struct Input {
    map: HashMap<(usize, usize), char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut map = HashMap::new();

        for line in input.lines() {
            let trace = parse_line(&line);

            for window in trace.windows(2) {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];

                for y in y1.min(y2)..=y1.max(y2) {
                    for x in x1.min(x2)..=x1.max(x2) {
                        map.insert((x, y), ROCK);
                    }
                }
            }
        }

        Ok(Input { map })
    }
}

/// Parse input of the format `x,y -> x,y -> x,y`.
fn parse_line(line: &str) -> Vec<(usize, usize)> {
    line.split(" -> ")
        .map(|value| {
            let (x, y) = value.split_once(",").unwrap();
            
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 93);
    }
}
