//! Advent of Code 2022: Day 20
//! https://adventofcode.com/2022/day/20

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const KEY: i64 = 811589153;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> i64 {
    let len = input.values.len() as i64;

    // i -> n mapping
    let mut forward: HashMap<i64, i64> = (0..len).map(|i| (i, i)).collect();
    // undoes i -> n mapping
    let mut reverse = forward.clone();

    for i in 0..len {
        let a = reverse[&i];
        let n = input.values[forward[&a] as usize];
        let b = (a + n).rem_euclid(len - 1);

        if a <= b {
            for x in a..b {
                // Swap indexes
                let v1 = forward[&x];
                let v2 = forward[&((x + 1) % len)];

                swap(&mut forward, x, (x + 1) % len);
                swap(&mut reverse, v1, v2);
            }
        } else {
            for x in (b..a).rev() {
                // Swap indexes
                let v1 = forward[&x];
                let v2 = forward[&((x + 1) % len)];

                swap(&mut forward, x, (x + 1) % len);
                swap(&mut reverse, v1, v2);
            }
        }
    }

    // Find the index of 0
    let (zero, _) = input.values.iter().copied().enumerate().find(|&(_, x)| x == 0).unwrap();
    let index = reverse[&(zero as i64)];
    
    let x = input.values[forward[&((index + 1000) % len)] as usize];
    let y = input.values[forward[&((index + 2000) % len)] as usize];
    let z = input.values[forward[&((index + 3000) % len)] as usize];

    x + y + z
}

fn part2(input: &Input) -> i64 {
    let values: Vec<_> = input.values.iter().copied().map(|n| n * KEY).collect();
    let len = input.values.len() as i64;

    // i -> n mapping
    let mut forward: HashMap<i64, i64> = (0..len).map(|i| (i, i)).collect();
    // undoes i -> n mapping
    let mut reverse = forward.clone();

    // This is extremely brute force (but works).
    // I'm almost certain the cycle repeats.
    for _ in 0..10 {
        for i in 0..len {
            let a = reverse[&i];
            let n = values[forward[&a] as usize];
            let b = (a + n).rem_euclid(len - 1);

            if a <= b {
                for x in a..b {
                    // Swap indexes
                    let v1 = forward[&x];
                    let v2 = forward[&((x + 1) % len)];

                    swap(&mut forward, x, (x + 1) % len);
                    swap(&mut reverse, v1, v2);
                }
            } else {
                for x in (b..a).rev() {
                    // Swap indexes
                    let v1 = forward[&x];
                    let v2 = forward[&((x + 1) % len)];

                    swap(&mut forward, x, (x + 1) % len);
                    swap(&mut reverse, v1, v2);
                }
            }
        }
    }

    // Find the index of 0
    let (zero, _) = values.iter().copied().enumerate().find(|&(_, x)| x == 0).unwrap();
    let index = reverse[&(zero as i64)];
    
    let x = values[forward[&((index + 1000) % len)] as usize];
    let y = values[forward[&((index + 2000) % len)] as usize];
    let z = values[forward[&((index + 3000) % len)] as usize];

    x + y + z
}

fn swap(map: &mut HashMap<i64, i64>, k1: i64, k2: i64) {
    let v1 = map.remove(&k1).unwrap();
    let v2 = map.remove(&k2).unwrap();

    map.insert(k1, v2);
    map.insert(k2, v1);
}


#[derive(Debug, Clone)]
struct Input {
    values: Vec<i64>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            values.push(line.parse::<i64>().unwrap());
        }

        Ok(Input { values })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 1623178306);
    }
}
