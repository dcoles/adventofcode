//! Advent of Code 2022: Day 13
//! https://adventofcode.com/2022/day/13

use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

use serde_json::Value;
use serde_json::json;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut correct: Vec<usize> = Vec::new();

    for (index, packets) in input.values.chunks_exact(2).enumerate() {
        let index = index + 1;
        let left = &packets[0];
        let right = &packets[1];

        println!("== Pair {} ==", index);
        println!("{:?}", left);
        println!("{:?}", right);
        println!();

        if left < right {
            correct.push(index);
        }
    }

    correct.into_iter().sum()
}

fn part2(input: &Input) -> usize {
    let mut packets = input.values.clone();

    let divider2: Item = Item(json!([[2]]));
    let divider6: Item = Item(json!([[6]]));

    // Divider packets
    packets.push(divider2.clone());
    packets.push(divider6.clone());

    packets.sort();

    packets.into_iter()
        .enumerate()
        .filter(|(_, p)| *p == divider2 || *p == divider6 )
        .map(|(i, _)| i + 1)
        .product()
}

/// A thin wrapper around `Value`.
#[derive(Clone, PartialEq, Eq)]
struct Item(Value);

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        value_cmp(&self.0, &other.0)
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item({})", serde_json::to_string(&self.0).unwrap())
    }
}

impl FromStr for Item {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map(Item)
    }
}

fn value_cmp(left: &Value, right: &Value) -> Ordering {
        match (left, right) {
            (Value::Array(ls), Value::Array(rs)) => {
                for i in 0.. {
                    if i == ls.len() && i == rs.len() {
                        // Ran out of items in both lists
                        return Ordering::Equal;
                    } else if i == ls.len() {
                        // Left list runs out of items
                        return Ordering::Less;
                    } else if i == rs.len() {
                        // Right list runs out of items
                        return Ordering::Greater;
                    } else {
                        // Check each item
                        let ord = Item(ls[i].clone()).cmp(&Item(rs[i].clone()));

                        if !matches!(ord, Ordering::Equal) {
                            return ord.clone();
                        }
                    }
                }

                unreachable!();
            },
            (Value::Array(_), Value::Number(r)) => {
                // Convert integer to a list
                let rs = json!([r]);

                value_cmp(left, &rs)
            },
            (Value::Number(l), Value::Array(_)) => {
                // Convert integer to a list
                let ls = json!([l]);

                value_cmp(&ls, right)
            },
            (Value::Number(l), Value::Number(r)) => {
                let l = l.as_u64().unwrap();
                let r = r.as_u64().unwrap();

                if l < r {
                    // Left integer lower than right
                    Ordering::Less
                } else if l > r {
                    // Left integer higher than right
                    Ordering::Greater
                } else {
                    // Check next part
                    Ordering::Equal
                }
            },
            _ => panic!("Can't compare {:?} and {:?}", left, right)
        }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Item>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.replace("\n\n", "\n").lines() {
            values.push(line.parse().unwrap());
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

        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 140);
    }
}
