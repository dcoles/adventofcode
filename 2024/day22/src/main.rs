//! Advent of Code 2024: Day 22
//! https://adventofcode.com/2024/day/22

use std::{fs, io};
use std::collections::{HashMap, HashSet};
use std::path::Path;

const ITERATIONS: usize = 2000;
const MODULUS: usize = 16777216;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example2.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //println!("{input:?}");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut sum = 0;

    for mut n in input.values.iter().copied() {
        for _ in 0..ITERATIONS {
            n = iterate(n);
        }

        sum += n;
    }

    sum
}

fn iterate(mut n: usize) -> usize {
    n ^= n * 64;
    n %= MODULUS;

    n ^= n / 32;
    n %= MODULUS;

    n ^= n * 2048;
    n %= MODULUS;

    n
}


fn part2(input: &Input) -> usize {
    let secrets: Vec<Vec<_>> = input.values.iter()
        .map(|&n| {
            let mut n = n;
            let mut list = vec![n];

            for _ in 0..ITERATIONS {
                n = iterate(n);
                list.push(n);
            }

            list
        })
        .collect();

    let prices: Vec<Vec<usize>> = secrets.iter()
        .map(|secrets| secrets.iter().map(|&n| n % 10).collect())
        .collect();

    let changes: Vec<Vec<i32>> = prices.iter()
        .map(|prices| {
            prices.windows(2)
                .map(|window| window[1] as i32 - window[0] as i32)
                .collect()
        })
        .collect();

    let mut sequence_prices = Vec::new();
    for (n, each) in changes.iter().enumerate() {
        let mut seq_prices = HashMap::new();
        for (m, window) in each.windows(4).enumerate() {
            if !seq_prices.contains_key(&window) {
                seq_prices.insert(window, prices[n][m + 4]);
            }
        }
        sequence_prices.push(seq_prices);
    }

    let all_sequences: HashSet<&[i32]> = HashSet::from_iter(
        sequence_prices.iter()
            .flat_map(|s| s.keys().map(|k| *k))
    );

    // This works though takes about 40 secs.
    // Clearly there is a better solution.
    let mut best = 0;

    for sequence in all_sequences {
        let mut total = 0;

        for seq_prices in &sequence_prices {
            if let Some(&price) = seq_prices.get(&sequence) {
                total += price;
            }
        }

        if total > best {
            best = total;
        }
    }

    best
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<usize>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;
        let values = input.lines().map(|s| s.parse().unwrap()).collect();

        Ok(Self { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 37327623);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 20506453102);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part2(&input), 23);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 2423);
    }
}
