//! Advent of Code 2021: Day 14
//! https://adventofcode.com/2021/day/14

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file("day14/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut polymer = input.template.clone();
    for _ in 0..10 {
        let mut new_polymer = Vec::new();
        for pair in polymer.windows(2) {
            new_polymer.push(pair[0]);
            if let Some(&c) = input.rules.get(pair) {
                new_polymer.push(c);
            }
        }
        new_polymer.push(*input.template.last().unwrap());

        polymer = new_polymer;
    }

    // Count elements
    let mut count: HashMap<_, usize> = HashMap::new();
    for c in polymer {
        *count.entry(c).or_default() += 1;
    };

    let mut sorted: Vec<_> = count.into_iter().collect();
    sorted.sort_by_key(|&(_, count) | count);

    // Most comment element subtract least comment element
    sorted.last().unwrap().1 - sorted.first().unwrap().1
}

fn part2(input: &Input) -> usize {
    // Key idea: Just keep track of polymer pairs counts for each step.
    // This is all we need to determine the quantities for the next step.
    let mut pairs: HashMap<_, usize> = HashMap::new();
    for pair in input.template.windows(2) {
        if let &[a, b, ..] = pair {
            *pairs.entry((a, b)).or_default() += 1;
        }
    }

    for _ in 0..40 {
        let mut new_pairs: HashMap<_, usize> = HashMap::new();
        for (&(a, b), &count) in pairs.iter() {
            if let Some(&c) = input.rules.get([a, b].as_ref()) {
                // C is inserted between A and B
                *new_pairs.entry((a, c)).or_default() += count;
                *new_pairs.entry((c, b)).or_default() += count;
            }
        }

        pairs = new_pairs;
    }

    // Count elements
    let mut count: HashMap<_, usize> = HashMap::new();
    for ((a, _), n) in pairs {
        *count.entry(a).or_default() += n;
    };

    // Make sure we count the last element!
    *count.entry(*input.template.last().unwrap()).or_default() += 1;

    let mut sorted: Vec<_> = count.into_iter().collect();
    sorted.sort_by_key(|&(_, count) | count);

    // Most comment element subtract least comment element
    sorted.last().unwrap().1 - sorted.first().unwrap().1
}

#[derive(Debug, Clone)]
struct Input {
    template: Vec<char>,
    rules: HashMap<Vec<char>, char>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let (template_chunk, rules_chunk) = input.split_once("\n\n").unwrap();

        let mut rules = HashMap::new();
        for line in rules_chunk.lines() {
            let (a, b) = line.split_once(" -> ").unwrap();
            rules.insert(a.chars().collect(), b.chars().next().unwrap());
        }

        Ok(Input { template: template_chunk.chars().collect(), rules })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&input), 1588);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&input), 2188189693529);
    }
}
