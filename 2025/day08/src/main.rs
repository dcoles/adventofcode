//! Advent of Code 2025: Day 8
//! <https://adventofcode.com/2025/day/8>

use std::collections::BTreeSet;
use std::{fs, io};
use std::path::Path;

use lib::vector::Vector;

type Vector3 = Vector<i64, 3>;

fn main() {
    let (input, n) = (Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input"), 1000);
    //let (input, n) = (Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input"), 10);
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input, n));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input, n: usize) -> usize {
    let (circuits, _) = connect_lights(input, n);

    let mut counts: Vec<usize> = circuits.iter().map(BTreeSet::len).collect();
    counts.sort_unstable();
    counts.reverse();

    counts.into_iter().take(3).product()
}

// Attempt to connect the first `n` closest pair of light boxes,
// returning a tuple of the resulting connected circuits and a list of meaningful connections
fn connect_lights(input: &Input, n: usize) -> (BTreeSet<BTreeSet<Vector3>>, Vec<(Vector3, Vector3)>) {
    let distances = all_pairs_distance(input);

    let mut connections = vec![];
    let mut circuits: BTreeSet<BTreeSet<Vector3>> = input.values.iter().copied()
        .map(|v| BTreeSet::from([v]))
        .collect();

    for (_, (a, b)) in distances.into_iter().take(n) {
        if circuits.len() == 1 {
            // Everything is connected!
            break;
        }

        let mut count = 0;
        let circuit = circuits.extract_if(.., |set| set.contains(&a) || set.contains(&b))
            .inspect(|_| count += 1)
            .reduce(|acc, x| { &acc | &x }).unwrap();

        if count == 2 {
            connections.push((a, b));
        }

        circuits.insert(circuit);
    }

    (circuits, connections)
}

fn all_pairs_distance(input: &Input) -> Vec<(i64, (Vector3, Vector3))> {
    let mut distances = Vec::new();

    for i in 0..input.values.len() {
        for j in (i + 1)..input.values.len() {
            let v1 = input.values[i];
            let v2 = input.values[j];

            let distance = (v1 - v2).magnitude_squared();

            distances.push((distance, (v1, v2)));
        }
    }

    distances.sort_by_key(|(d, _)| *d);

    distances
}

fn part2(input: &Input) -> i64 {
    let (_, connections) = connect_lights(input, usize::MAX);

    connections.last().map(|(v1, v2)| v1[0] * v2[0]).unwrap()
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Vector3>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|line| {
            let value: [i64; 3] = line.split(',')
                .map(|val| val.parse().expect("should be valid integer"))
                .collect::<Vec<_>>()
                .try_into()
                .expect("should be three values");

            Vector3::new(value)
        }).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input, 10), 40);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input, 1000), 47040);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 25272);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 4884971896);
    }
}
