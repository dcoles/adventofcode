//! Advent of Code 2025: Day 7
//! <https://adventofcode.com/2025/day/7>

use std::collections::{BTreeMap, BTreeSet};
use std::{fs, io};
use std::path::Path;

type Pos = lib::vector::Vector<usize, 2>;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut count = 0;

    let mut beams: BTreeSet<Pos> = [input.start + Pos::new([0, 1])].into_iter().collect();

    for _ in 0..input.n_lines {
        let mut new_beams = BTreeSet::new();
        for pos in beams {
            let new_pos = pos + Pos::new([0, 1]);

            if input.splitters.contains(&pos) {
                new_beams.insert(new_pos - Pos::new([1, 0]));
                new_beams.insert(new_pos + Pos::new([1, 0]));
                count += 1;
            } else {
                new_beams.insert(new_pos);
            }
        }
        beams = new_beams;
    }

    count
}

fn part2(input: &Input) -> usize {
    let first_pos = input.start + Pos::new([0, 1]);
    let mut beams: BTreeMap<Pos, usize> = [(first_pos, 1)].into_iter().collect();

    for _ in 0..input.n_lines {
        let mut new_beams = BTreeMap::new();
        for (pos, n) in beams {
            let new_pos = pos + Pos::new([0, 1]);

            if input.splitters.contains(&pos) {
                *new_beams.entry(new_pos - Pos::new([1, 0])).or_default() += n;
                *new_beams.entry(new_pos + Pos::new([1, 0])).or_default() += n;
            } else {
                *new_beams.entry(new_pos).or_default() += n;
            }
        }
        beams = new_beams;
    }

    beams.values().sum()
}


#[derive(Debug, Clone)]
struct Input {
    start: Pos,
    splitters: BTreeSet<Pos>,
    n_lines: usize,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut start = Pos::default();
        let mut splitters = BTreeSet::new();
        let mut n_lines = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Pos::new([x, y]);
                match c {
                    'S' => start = pos,
                    '^' => { splitters.insert(pos); },
                    _ => (),
                }
            }

            n_lines += 1;
        }

        Ok(Self { start, splitters, n_lines })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 1594);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 40);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 15650261281478);
    }
}
