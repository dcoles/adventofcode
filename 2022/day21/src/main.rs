//! Advent of Code 2022: Day 21
//! https://adventofcode.com/2022/day/21

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const ROOT: &str = "root";
const HUMAN: &str = "humn";

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");
    //let input = Input::from_file(format!("{}/example1.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> i64 {
    evaluate(input)[ROOT]
}

fn part2(input: &Input) -> i64 {
    // Evaluate the tree as before
    let mut eval = evaluate(input);
    eval.remove(HUMAN);

    // Build a new inverted tree
    let mut tree = HashMap::new();
    let mut parent = HashMap::new();
    for monkey in input.monkeys.values() {
        if monkey.name == ROOT {
            match &monkey.job {
                Job::Add(a, b) | Job::Subtract(a, b) | Job::Multiply(a, b) | Job::Divide(a, b) => {
                    tree.insert(a.to_string(), Job::Root(b.to_string()));
                    tree.insert(b.to_string(), Job::Root(a.to_string()));
                },
                _ => panic!("root is a constant?!?"),
            };

            continue;
        }

        match &monkey.job {
            Job::Add(a, b) => {
                tree.insert(a.clone(), Job::Subtract(monkey.name.clone(), b.to_string()));
                tree.insert(b.clone(), Job::Subtract(monkey.name.clone(), a.to_string()));
                parent.insert(a.clone(), monkey.name.clone());
                parent.insert(b.clone(), monkey.name.clone());
            },
            Job::Subtract(a, b) => {
                tree.insert(a.clone(), Job::Add(monkey.name.clone(), b.to_string()));
                tree.insert(b.clone(), Job::Subtract(a.to_string(), monkey.name.clone()));
                parent.insert(a.clone(), monkey.name.clone());
                parent.insert(b.clone(), monkey.name.clone());
            },
            Job::Multiply(a, b) => {
                tree.insert(a.clone(), Job::Divide(monkey.name.clone(), b.to_string()));
                tree.insert(b.clone(), Job::Divide(monkey.name.clone(), a.to_string()));
                parent.insert(a.clone(), monkey.name.clone());
                parent.insert(b.clone(), monkey.name.clone());
            },
            Job::Divide(a, b) => {
                tree.insert(a.clone(), Job::Multiply(monkey.name.clone(), b.to_string()));
                tree.insert(b.clone(), Job::Divide(a.to_string(), monkey.name.clone()));
                parent.insert(a.clone(), monkey.name.clone());
                parent.insert(b.clone(), monkey.name.clone());
            },
            _ => (),
        }
    }

    // Walk the stack to find which nodes we need re-evaluate
    let mut stack = vec![HUMAN];
    while let Some(p) = parent.get(*stack.last().unwrap()) {
        stack.push(p.as_str());
    }

    // Evaluate the tree
    while let Some(current) = stack.pop() {
        let result = match &tree[current] {
            Job::Add(a, b) => {
                eval[a] + eval[b]
            },
            Job::Subtract(a, b) => {
                eval[a] - eval[b]
            },
            Job::Multiply(a, b) => {
                eval[a] * eval[b]
            },
            Job::Divide(a, b) => {
                eval[a] / eval[b]
            },
            Job::Constant(n) => *n,
            Job::Root(a) => eval[a],
        };

        eval.insert(current.to_string(), result);
    }

    eval[HUMAN]
}

/// Evaluate the tree from "root" down
fn evaluate(input: &Input) -> HashMap<String, i64> {
    let mut unvisited = vec![ROOT];
    let mut stack = vec![];
    while let Some(current) = unvisited.pop() {
        match &input.monkeys[current].job {
            Job::Add(a, b) => {
                unvisited.push(a.as_str());
                unvisited.push(b.as_str());
            },
            Job::Subtract(a, b) => {
                unvisited.push(a.as_str());
                unvisited.push(b.as_str());
            },
            Job::Multiply(a, b) => {
                unvisited.push(a.as_str());
                unvisited.push(b.as_str());
            },
            Job::Divide(a, b) => {
                unvisited.push(a.as_str());
                unvisited.push(b.as_str());
            },
            _ => (),
        }

        stack.push(current.clone());
    }

    let mut eval = HashMap::new();
    while let Some(current) = stack.pop() {
        let result = match &input.monkeys[current].job {
            Job::Add(a, b) => {
                eval[a] + eval[b]
            },
            Job::Subtract(a, b) => {
                eval[a] - eval[b]
            },
            Job::Multiply(a, b) => {
                eval[a] * eval[b]
            },
            Job::Divide(a, b) => {
                eval[a] / eval[b]
            },
            Job::Constant(n) => *n,
            Job::Root(a) => eval[a],
        };

        eval.insert(current.to_string(), result);
    }

    eval
}

// 🐒
#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: Job,
}

/// Possible jobs
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Job {
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Constant(i64),
    Root(String),
}


#[derive(Debug, Clone)]
struct Input {
    monkeys: HashMap<String, Monkey>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut monkeys = HashMap::new();
        for line in input.lines() {
            let (name, job) = line.split_once(": ").unwrap();

            let job = if job.contains("+") {
                let (a, b) = job.split_once(" + ").unwrap();

                Job::Add(a.to_string(), b.to_string())
            } else if job.contains("-") {
                let (a, b) = job.split_once(" - ").unwrap();

                Job::Subtract(a.to_string(), b.to_string())
            } else if job.contains("*") {
                let (a, b) = job.split_once(" * ").unwrap();

                Job::Multiply(a.to_string(), b.to_string())
            } else if job.contains("/") {
                let (a, b) = job.split_once(" / ").unwrap();

                Job::Divide(a.to_string(), b.to_string())
            } else {
                let n = job.parse::<i64>().unwrap();

                Job::Constant(n)
            };

            monkeys.insert(name.to_string(),Monkey { name: name.to_string(), job});
        }

        Ok(Input { monkeys })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 152);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 301);
    }
}
