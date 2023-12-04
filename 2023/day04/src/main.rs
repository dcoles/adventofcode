//! Advent of Code 2023: Day 4
//! https://adventofcode.com/2023/day/4

use std::collections::HashMap;
use std::collections::HashSet;
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

fn part1(input: &Input) -> u32 {
    let mut total = 0;

    for card in &input.cards {
        let matches = &card.numbers & &card.winning_numbers;
        let points = match matches.len() {
            0 => 0,
            n => 1 << (n - 1),
        };

        total += points;
    }

    total
}

fn part2(input: &Input) -> usize {
    let n_matches: HashMap<usize, usize> = input.cards.iter().map(|card| (card.id, (&card.numbers & &card.winning_numbers).len())).collect();
    let mut num_cards: HashMap<usize, usize> = input.cards.iter().map(|card| (card.id, 1)).collect();

    for card in &input.cards {
        for n in 1..=n_matches[&card.id] {
            *num_cards.get_mut(&(card.id + n)).unwrap() += num_cards[&card.id];
        }
    }

    num_cards.values().copied().sum()
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

#[derive(Debug, Clone)]
struct Input {
    cards: Vec<Card>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut cards = Vec::new();
        for (n, line) in input.lines().enumerate() {
            let values: Vec<_> = line.split_ascii_whitespace().collect();
            let p = values.iter().position(|s| *s == "|").unwrap();
            let winning_numbers = values[2..p].iter().map(|s| s.parse::<u32>().unwrap()).collect();
            let numbers = values[p+1..].iter().map(|s| s.parse::<u32>().unwrap()).collect();

            cards.push(Card { id: n + 1, numbers, winning_numbers });
        }

        Ok(Input { cards })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 23941);
    }

    #[test]
    fn test_part2_example1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 30);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 5571760);
    }
}
