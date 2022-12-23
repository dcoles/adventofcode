//! Advent of Code 2022: Day 23
//! https://adventofcode.com/2022/day/23

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;

type Pos = (i32, i32);
const ELF: char = '#';
const DIRECTIONS: [[Pos; 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)], // N
    [(-1, 1), (0, 1), (1, 1)], // S
    [(-1, -1), (-1, 0), (-1, 1)], // W
    [(1, -1), (1, 0), (1, 1)], // E
];

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let (elves, _) = simulate(&input.elves, 10);

    let min_x = elves.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = elves.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = elves.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = elves.iter().map(|&(_, y)| y).max().unwrap();

    let dx = (max_x - min_x + 1) as usize;
    let dy = (max_y - min_y + 1) as usize;

    dx * dy - elves.len()
}

fn part2(input: &Input) -> usize {
    let (_, rounds) = simulate(&input.elves, usize::MAX);

    rounds
}

fn simulate(elves: &HashSet<Pos>, max_rounds: usize) -> (HashSet<Pos>, usize) {
    let mut elves = elves.clone();
    let mut total_rounds = 0;
    let mut directions = DIRECTIONS.to_vec();

    for round in 1..=max_rounds {
        let mut new_elves = HashSet::new();
        let mut proposals = Vec::new();
        total_rounds = round;

        // First half: Proposal
        for &pos in &elves {
            if directions.iter().all(|dirs| dirs.iter().all(|&(dx, dy)| !elves.contains(&(pos.0 + dx, pos.1 + dy)))) {
                // This elf does nothing
                new_elves.insert(pos);
                continue;
            }

            // Can we find a position to move to?
            let mut proposal = None;
            for dirs in &directions {
                if dirs.iter().all(|&(dx, dy)| !elves.contains(&(pos.0 + dx, pos.1 + dy))) {
                    proposal = Some((pos, (pos.0 + dirs[1].0, pos.1 + dirs[1].1)));
                    break;
                }
            }

            if let Some(proposed) = proposal {
                // Propose moving to new position
                proposals.push(proposed);
            } else {
                // Stay where we are
                new_elves.insert(pos);
            }
        }

        if proposals.is_empty() {
            break;
        }

        // Second half: Execution
        let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
        for &(_, to) in &proposals {
            *counts.entry(to).or_default() += 1;
        }

        for &(from, to) in &proposals {
            if counts[&to] > 1 {
                // Stays in the current position
                new_elves.insert(from);
            } else {
                // Moves to new position
                new_elves.insert(to);
            }
        }

        assert_eq!(new_elves.len(), elves.len());

        elves = new_elves;
        directions.rotate_left(1);
    }

    (elves, total_rounds)
}

#[derive(Debug, Clone)]
struct Input {
    elves: HashSet<Pos>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut elves = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != ELF {
                    continue;
                }

                elves.insert((x as i32, y as i32));
            }
        }

        Ok(Input { elves })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example3.txt").unwrap();

        assert_eq!(part1(&input), 110);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example3.txt").unwrap();

        assert_eq!(part2(&input), 20);
    }
}
