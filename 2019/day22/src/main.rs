use std::path::Path;
use std::fs;
use std::collections::VecDeque;

const DEAL_NEW_STACK: &str = "deal into new stack";
const CUT: &str = "cut ";
const DEAL_WITH_INCREMENT: &str = "deal with increment ";
const N_CARDS: usize = 10007;

fn main() {
    let input = read_input("input.txt");

    // Part 1
    // Front is the top of the deck, back is the bottom
    let mut deck: VecDeque<_> = (0..N_CARDS).map(Card).collect();
    for technique in input {
        match technique {
            Technique::DealNewStack => {
                let mut stack: Vec<_> = deck.drain(..).collect();
                stack.reverse();
                deck.extend(stack.into_iter());
            },
            Technique::CutCards(n) => {
                if n >= 0 {
                    for _ in 0..n {
                        let card = deck.pop_front().unwrap();
                        deck.push_back(card)
                    }
                } else {
                    for _ in 0..n.abs() {
                        let card = deck.pop_back().unwrap();
                        deck.push_front(card)
                    }
                }
            },
            Technique::DealWithIncrement(n) => {
                let mut table = vec![None; deck.len()];
                for (i, card) in deck.into_iter().enumerate() {
                    let index = (i * n) % table.len();
                    table[index] = Some(card);
                }
                deck = table.into_iter().map(|c| c.unwrap()).collect();
            },
        }
    }

    let (position, _) = deck.iter().copied().enumerate().find(|&(_, card)| card == Card(2019)).unwrap();
    println!("Part 1: Postion of card 2019: {}", position);
}

fn read_input<T: AsRef<Path>>(path: T) -> Vec<Technique> {
    let contents = fs::read_to_string(path).expect("Failed to read input");

    let mut techniques = Vec::new();
    for line in contents.lines() {
        let technique = match line {
            DEAL_NEW_STACK => Technique::DealNewStack,
            line if line.starts_with(CUT) => {
                let n = line[CUT.len()..].parse().expect("Failed to parse value");

                Technique::CutCards(n)
            },
            line if line.starts_with(DEAL_WITH_INCREMENT) => {
                let n = line[DEAL_WITH_INCREMENT.len()..].parse().expect("Failed to parse value");

                Technique::DealWithIncrement(n)
            }
            line => panic!("Unknown line: {}", line),
        };

        techniques.push(technique);
    }

    techniques
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Card(usize);

#[derive(Debug)]
enum Technique {
    DealNewStack,
    CutCards(i32),
    DealWithIncrement(usize),
}
