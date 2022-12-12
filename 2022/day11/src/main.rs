//! Advent of Code 2022: Day 11
//! https://adventofcode.com/2022/day/11

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut inspections: HashMap<usize, usize> = HashMap::new();
    let mut monkeys = input.monkeys.clone();
    for _round in 0..20 {
        for m in 0..monkeys.len() {

            let starting_items: Vec<_> = monkeys[m].starting_items.drain(..).collect();
            for item in starting_items {
                let worry = match monkeys[m].operation {
                    ('*', None) => item * item,
                    ('+', None) => item + item,
                    ('*', Some(x)) => item * x,
                    ('+', Some(x)) => item + x,
                    op => panic!("Unknown op: {:?}", op),
                };

                *inspections.entry(m).or_default() += 1;
                let worry = worry / 3;

                let next = if worry % monkeys[m].test == 0 {
                    // Divisible
                    monkeys[m].if_true
                } else {
                    // Not-Divisible
                    monkeys[m].if_false
                };

                monkeys[next].starting_items.push(worry);
            }
        }
    }

    // What is the level of monkey business after 20 rounds of stuff-slinging simian shenanigans?
    monkey_business(&inspections)
}

fn part2(input: &Input) -> usize {
    let mut inspections: HashMap<usize, usize> = HashMap::new();
    let mut monkeys = input.monkeys.clone();

    // Do everything modulo the common primes
    let modulus: i64 = monkeys.iter().map(|m| m.test).product();

    for _round in 0..10000 {
        for m in 0..monkeys.len() {
            let starting_items: Vec<_> = monkeys[m].starting_items.drain(..).collect();
            let test = monkeys[m].test;
            for item in starting_items {
                let item = item % modulus;
                let mut worry = match monkeys[m].operation {
                    ('*', None) => item * item,
                    ('+', None) => item + item,
                    ('*', Some(x)) => item * x,
                    ('+', Some(x)) => item + x,
                    op => panic!("Unknown op: {:?}", op),
                };

                *inspections.entry(m).or_default() += 1;

                worry %= modulus;

                let next = if worry % test == 0 {
                    // Divisible
                    monkeys[m].if_true
                } else {
                    // Not-Divisible
                    monkeys[m].if_false
                };

                monkeys[next].starting_items.push(worry);
            }
        }
    }

    // what is the level of monkey business after 10000 rounds?
    monkey_business(&inspections)
}

fn monkey_business(inspections: &HashMap<usize, usize>) -> usize {
    let mut most_active: Vec<_> = inspections.iter().map(|(&m, &n)| (m, n)).collect();
    most_active.sort_by_key(|(_, n)| *n);

    let most_active: Vec<_> = most_active.iter().rev().take(2).collect();

    most_active[0].1 * most_active[1].1
}

#[derive(Debug, Clone)]
struct Input {
    monkeys: Vec<Monkey>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut monkeys = Vec::new();
        for (id, monkey) in input.split("\n\n").enumerate() {
            let mut starting_items = Vec::new();
            let mut operation = ('+', None);
            let mut test = 0;
            let mut if_true = 0;
            let mut if_false = 0;

            for line in monkey.lines() {
                match line {
                    line if line.starts_with("  Starting items:") => {
                        let (_, values) = line.split_once(": ").unwrap();
                        starting_items = values.split(", ").map(|x| x.parse::<i64>().unwrap()).collect();
                    },
                    line if line.starts_with("  Operation:") => {
                        let (_, values) = line.split_once(": ").unwrap();
                        let (_, ops) = values.split_once(" = ").unwrap();
                        let mut ops = ops.split_ascii_whitespace();
                        let _left = ops.next().unwrap();
                        let op = ops.next().unwrap().chars().next().unwrap();
                        let right = ops.next().map(|x| x.parse::<i64>().ok()).unwrap();

                        operation = (op, right);
                    },
                    line if line.starts_with("  Test:") => {
                        test = line.split_ascii_whitespace().last().map(|x| x.parse::<i64>().unwrap()).unwrap();
                    },
                    line if line.starts_with("    If true:") => {
                        if_true = line.split_ascii_whitespace().last().map(|x| x.parse::<usize>().unwrap()).unwrap();
                    },
                    line if line.starts_with("    If false:") => {
                        if_false = line.split_ascii_whitespace().last().map(|x| x.parse::<usize>().unwrap()).unwrap();
                    },
                    _ =>(),
                }
            }

            monkeys.push(Monkey { id, starting_items, operation, test, if_true, if_false });
        }

        Ok(Input { monkeys })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey {
    id: usize,
    starting_items: Vec<i64>,
    operation: (char, Option<i64>),
    test: i64,
    if_true: usize,
    if_false: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 2713310158);
    }
}
