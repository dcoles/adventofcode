//! Advent of Code 2022: Day 13
//! https://adventofcode.com/2022/day/13

use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

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

    let divider2: Item = "[[2]]".parse().unwrap();
    let divider6: Item = "[[6]]".parse().unwrap();

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

/// An Item (either a List or a Value).
#[derive(Clone, PartialEq, Eq, Ord)]
enum Item {
    List(Vec<Item>),
    Value(u64),
}

impl Item {
    fn push(&mut self, item: Item) -> &mut Item {
        match self {
            Item::List(xs) => {
                xs.push(item);

                xs.last_mut().unwrap()
            }
            _ => panic!("Tried to push to a value"),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::List(ls), Item::List(rs)) => {
                for i in 0.. {
                    if i == ls.len() && i == rs.len() {
                        // Ran out of items in both lists
                        return Some(Ordering::Equal);
                    } else if i == ls.len() {
                        // Left list runs out of items
                        return Some(Ordering::Less);
                    } else if i == rs.len() {
                        // Right list runs out of items
                        return Some(Ordering::Greater)
                    } else {
                        // Check each item
                        let ord = ls[i].partial_cmp(&rs[i]);

                        if !matches!(ord, Some(Ordering::Equal)) {
                            return ord;
                        }
                    }
                }

                unreachable!();
            },
            (Item::List(_), Item::Value(r)) => {
                // Convert integer to a list
                let rs = Item::List(vec![Item::Value(*r)]);

                self.partial_cmp(&rs)
            },
            (Item::Value(l), Item::List(_)) => {
                // Convert integer to a list
                let ls = Item::List(vec![Item::Value(*l)]);

                ls.partial_cmp(other)
            },
            (Item::Value(l), Item::Value(r)) => {
                if l < r {
                    // Left integer lower than right
                    Some(Ordering::Less)
                } else if l > r {
                    // Left integer higher than right
                    Some(Ordering::Greater)
                } else {
                    // Check next part
                    Some(Ordering::Equal)
                }
            },
        }
    }
}

impl FromStr for Item {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Item> = vec![];

        let mut value = None;
        for c in s.chars() {
            if value.is_some() && !c.is_ascii_digit() {
                stack.last_mut().unwrap().push(Item::Value(value.unwrap()));
                value = None;
            }

            match c {
                '[' => {
                    stack.push(Item::List(vec![]));
                },
                ']' => {
                    let item = stack.pop().unwrap();

                    if let Some(last) = stack.last_mut() {
                        last.push(item);
                    } else {
                        return Ok(item);
                    }
                },
                c if c.is_ascii_digit() => {
                    let d = c.to_digit(10).unwrap().into();
                    if let Some(value) = &mut value {
                        *value = *value * 10 + d;
                    } else {
                        value = Some(d);
                    }
                },
                _ => (),
            }
        }

        Err(io::Error::new(io::ErrorKind::Other, "Truncated input"))
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::List(xs) => write!(f, "{:?}", xs),
            Item::Value(x) => write!(f, "{}", x),
        }
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
            values.push(line.parse::<Item>().unwrap());
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
