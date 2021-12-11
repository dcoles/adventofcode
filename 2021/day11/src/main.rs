//! Advent of Code 2021: Day 11
//! https://adventofcode.com/2021/day/11

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

const SIZE: usize = 10;
const STEPS: usize = 100;

type Pos = (i32, i32);

fn main() {
    let input = read_input_from_file("day11/input.txt").expect("failed to read input");

    // Part 1
    println!("{}", part1(&input));

    // Part 2
    println!("{}", part2(&input));
}

fn part1(input: &HashMap<Pos, u32>) -> usize {
    let mut flashes = 0;

    let mut map = input.clone();
    for _ in 0..STEPS {
        for val in map.values_mut() {
            *val += 1;
        }

        let mut flashed = HashSet::new();
        loop {
            let energized: Vec<_> = map.iter()
                .filter_map(|(&pos, &n)| if n > 9 { Some(pos) } else { None })
                .collect();

            if energized.is_empty() {
                break;
            }

            for pos in energized {
                // Flash!
                map.insert(pos, 0);
                flashed.insert(pos);

                for adj in adjacent(&map, pos).into_iter().filter(|p| !flashed.contains(p)) {
                    *map.get_mut(&adj).unwrap() += 1;
                }
            }
        }

        flashes += flashed.len();
    }

    flashes
}

fn part2(input: &HashMap<Pos, u32>) -> usize {
    let mut map = input.clone();
    for n in 1.. {
        for val in map.values_mut() {
            *val += 1;
        }

        let mut flashed = HashSet::new();
        loop {
            let energized: Vec<_> = map.iter()
                .filter_map(|(&pos, &n)| if n > 9 { Some(pos) } else { None })
                .collect();

            if energized.is_empty() {
                break;
            }

            for pos in energized {
                // Flash!
                map.insert(pos, 0);
                flashed.insert(pos);

                for adj in adjacent(&map, pos).into_iter().filter(|p| !flashed.contains(p)) {
                    *map.get_mut(&adj).unwrap() += 1;
                }
            }
        }

        if flashed.len() == SIZE * SIZE {
            // All flashed simultaniously
            return n;
        }
    }

    unreachable!();
}

fn adjacent(map: &HashMap<Pos, u32>, pos: Pos) -> Vec<Pos> {
    let mut adjacent = Vec::new();
    for (dx, dy) in [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)] {
        let adj = (pos.0 + dx, pos.1 + dy);
        if map.contains_key(&adj) {
            adjacent.push(adj);
        }
    }

    adjacent
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<HashMap<Pos, u32>> {
    let input = fs::read_to_string(path)?;

    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c.to_digit(10).unwrap());
        }
    }

    Ok(map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 1656);
    }

    #[test]
    fn test_part2() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 195);
    }
}
