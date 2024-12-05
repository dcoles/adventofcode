//! Advent of Code 2024: Day 5
//! https://adventofcode.com/2024/day/5

use std::{fs, io};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
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
    let mut sum = 0;

    for update in &input.updates {
        if is_ordered(&update, &input.rules) {
            // pick middle value
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn is_ordered(update: &[usize], rules: &[(usize, usize)]) -> bool {
    let positions: BTreeMap<usize, usize> = update.iter().copied().enumerate().map(|(n, val)| (val, n)).collect();

    // Check that all rules are unbroken
    rules.iter().copied()
        .all(|(a, b)| match (positions.get(&a), positions.get(&b)) {
            (Some(&ia), Some(&ib)) => ia < ib,
            _ => true,
        })
}

fn part2(input: &Input) -> usize {
    let rules: HashSet<_> = input.rules.iter().copied().collect();
    let mut sum = 0;

    for update in input.updates.iter() {
        if !is_ordered(&update, &input.rules) {
            let mut pages = update.clone();

            pages.sort_by(|&l, &r| {
                if rules.contains(&(l, r)) {
                    Ordering::Less
                } else if rules.contains(&(r, l)) {
                    Ordering::Greater
                } else {
                    panic!("Unknown total ordering!");
                }
            });

            // Pick middle value
            sum += pages[pages.len() / 2];
        }
    }

    sum
}

#[derive(Debug, Clone)]
struct Input {
    rules: Vec<(usize, usize)>,
    updates: Vec<Vec<usize>>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let (section1, section2) = input.split_once("\n\n").unwrap();

        let rules: Vec<(usize, usize)> = section1.lines()
            .map(|line| {
                let (a, b) = line.trim().split_once('|').unwrap();

                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();

        let updates: Vec<Vec<usize>> = section2.lines()
            .map(|line| {
                line.trim().split(',').map(|s| s.parse().unwrap()).collect()
            })
            .collect();

        Ok(Self { rules, updates })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 143);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 6498);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 123);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 5017);
    }
}
