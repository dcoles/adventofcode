//! Advent of Code 2023: Day 7
//! https://adventofcode.com/2023/day/7

use std::cmp::Ordering;
use std::collections::BTreeMap;
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
    let mut bids = input.values.clone();
    bids.sort_by(|(left_hand, _), (right_hand, _)| {
        let left_type = TypeOfHand::from_cards(left_hand, false);
        let right_type = TypeOfHand::from_cards(right_hand, false);

        match left_type.cmp(&right_type) {
            Ordering::Equal => left_hand.cmp(right_hand),
            ord => ord,
        }
    });

    bids.into_iter().enumerate().map(|(n, (_, bid))| (n + 1) * bid).sum()
}

fn part2(input: &Input) -> usize {
    let mut bids = input.values.clone();
    bids.sort_by(|(left_hand, _), (right_hand, _)| {
        let left_type = TypeOfHand::from_cards(left_hand, true);
        let right_type = TypeOfHand::from_cards(right_hand, true);

        match left_type.cmp(&right_type) {
            Ordering::Equal => cmp_cards_with_wildcards(left_hand, right_hand),
            ord => ord,
        }
    });

    bids.into_iter().enumerate().map(|(n, (_, bid))| (n + 1) * bid).sum()
}

fn cmp_cards_with_wildcards(a: &[Card], b: &[Card]) -> Ordering {
    for (card_a, card_b) in a.iter().zip(b) {
        if matches!(card_a, Card::J) && !matches!(card_b, Card::J) {
            // Joker is always the weakest card
            return Ordering::Less;
        } else if !matches!(card_a, Card::J) && matches!(card_b, Card::J) {
            // Joker is always the weakest card
            return Ordering::Greater;
        } else {
            match card_a.cmp(card_b) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                _ => (),
            }
        }
    }

    Ordering::Equal
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum TypeOfHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TypeOfHand {
    fn from_cards(cards: &[Card], jokers_wild: bool) -> Self {
        let n_wildcards = if jokers_wild {
            cards.iter().filter(|c| matches!(c, Card::J)).count()
        } else {
            0
        };

        let mut counts: BTreeMap<Card, usize> = BTreeMap::new();
        for &card in cards {
            if jokers_wild && matches!(card, Card::J) {
                continue;
            }

            *counts.entry(card).or_default() += 1;
        }

        let mut count_values: Vec<_> = counts.values().copied().collect();
        count_values.sort();
        let max_of_a_kind = count_values.pop().unwrap_or_default();
        let next_max_of_a_kind = count_values.pop().unwrap_or_default();

        match max_of_a_kind + n_wildcards {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => if next_max_of_a_kind == 2 {
                Self::FullHouse
            } else {
                Self::ThreeOfAKind
            },
            2 => if next_max_of_a_kind == 2 {
                Self::TwoPair
            } else {
                Self::OnePair
            },
            _ => Self::HighCard,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    N(u32),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            n if n.is_ascii_digit() => Self::N(n.to_digit(10).unwrap()),
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => panic!("unknown card {:?}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<(Vec<Card>, usize)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            let (cards, n) = line.split_once(" ").unwrap();
            let cards = cards.chars().map(Card::from_char).collect();

            values.push((cards, n.parse().unwrap()));
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

        assert_eq!(part1(&input), 6440);
    }

    #[test]
    fn test_part1_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part1(&input), 250957639);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 5905);
    }

    #[test]
    fn test_part2_solution() {
        let input = Input::from_file("input.txt").unwrap();

        assert_eq!(part2(&input), 251515496);
    }
}
