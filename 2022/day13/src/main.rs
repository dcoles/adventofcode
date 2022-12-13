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

        if check(&left, &right) == Check::Correct {
            correct.push(index);
        }
    }

    correct.into_iter().sum()
}

fn part2(input: &Input) -> usize {
    let mut packets = input.values.clone();

    // Divider packets
    packets.push(Item::List(vec![Item::List(vec![Item::Value(2)])]));
    packets.push(Item::List(vec![Item::List(vec![Item::Value(6)])]));

    packets.sort_by(|left, right| match check(left, right) {
        Check::Correct => Ordering::Less,
        Check::Incorrect => Ordering::Greater,
        Check::Inconclusive => Ordering::Equal,
    });

    packets.into_iter()
        .enumerate()
        .filter(|(_, p)| is_divider(p))
        .map(|(i, _)| i + 1)
        .product()
}

/// Is this a Divider packet (`[[2]]` or `[[6]]`)
fn is_divider(packet: &Item) -> bool {
    if let Item::List(l1) = packet {
        if l1.len() != 1 {
            return false;
        }

        if let Item::List(l2) = &l1[0] {
            if l2.len() != 1 {
                return false;
            }

            if let Item::Value(v) = &l2[0] {
                return *v == 2 || *v == 6
            }
        }
    }

    return false;
}

/// Order packets
fn check(left: &Item, right: &Item) -> Check {
    match (left, right) {
        (Item::List(ls), Item::List(rs)) => {
            for i in 0.. {
                if i == ls.len() && i == rs.len() {
                    // Ran out of items in both lists
                    return Check::Inconclusive;
                } else if i == ls.len() {
                    // Left list runs out of items
                    return Check::Correct
                } else if i == rs.len() {
                    // Right list runs out of items
                    return Check::Incorrect
                } else {
                    // Check each item
                    let c = check(&ls[i], &rs[i]);

                    if c.conclusive() {
                        return c;
                    }
                }
            }

            unreachable!();
        },
        (Item::List(_), Item::Value(r)) => {
            // Convert integer to a list
            let mut rs = Item::list();
            rs.push(Item::Value(*r));

            check(left, &rs)
        },
        (Item::Value(l), Item::List(_)) => {
            // Convert integer to a list
            let mut ls = Item::list();
            ls.push(Item::Value(*l));

            check(&ls, right)
        },
        (Item::Value(l), Item::Value(r)) => {
            if l < r {
                // Left integer lower than right
                Check::Correct
            } else if l > r {
                // Left integer higher than right
                Check::Incorrect
            } else {
                // Check next part
                Check::Inconclusive
            }
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Check {
    Correct,
    Incorrect,
    Inconclusive,
}

impl Check {
    fn conclusive(self) -> bool {
        !matches!(self, Check::Inconclusive)
    }
}

#[derive(Clone)]
enum Item {
    List(Vec<Item>),
    Value(u64),
}

impl Item {
    fn list() -> Self {
        Item::List(Vec::new())
    }

    fn value(x: u64) -> Self {
        Item::Value(x)
    }

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

impl FromStr for Item {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack: Vec<Item> = vec![];

        let mut value = None;
        for c in s.chars() {
            if value.is_some() && !c.is_ascii_digit() {
                stack.last_mut().unwrap().push(Item::value(value.unwrap()));
                value = None;
            }

            match c {
                '[' => {
                    stack.push(Item::list());
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
