use std::path::Path;
use std::fs;
use std::collections::{VecDeque, HashSet};

fn main() {
    let (deck1, deck2) = read_input("input.txt");

    // Part 1
    let mut combat = Combat::new(deck1.clone(), deck2.clone());
    combat.play();
    println!("Part 1: SCORE {}", combat.score());
    println!();

    // Part 2
    let mut combat = RecursiveCombat::new(deck1.clone(), deck2.clone());
    combat.play();
    println!("Part 2: SCORE {}", combat.score());
}

fn read_input<T: AsRef<Path>>(path: T) -> (VecDeque<u64>, VecDeque<u64>) {
    let input = fs::read_to_string(path).expect("Failed to read input");

    let mut players = input.split("\n\n");

    let deck1 = players.next().expect("Missing player1 ")
        .lines()
        .skip(1)
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect();

    let deck2 = players.next().expect("Missing player 2")
        .lines()
        .skip(1)
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect();

    (deck1, deck2)
}

#[derive(Debug)]
struct Combat {
    deck1: VecDeque<u64>,
    deck2: VecDeque<u64>,
}

impl Combat {
    fn new(deck1: VecDeque<u64>, deck2: VecDeque<u64>) -> Self {
        Combat { deck1, deck2 }
    }

    fn play(&mut self) {
        let mut round = 0;
        while !self.is_winner() {
            round += 1;

            println!("-- Round {} --", round);
            println!("Player 1's deck: {:?}", self.deck1);
            println!("Player 2's deck: {:?}", self.deck2);

            self.play_round();
            println!();
        }

        println!();
        println!("== Post-game results ==");
        println!("Player 1's deck: {:?}", self.deck1);
        println!("Player 2's deck: {:?}", self.deck2);
    }

    fn play_round(&mut self) {
        let player1_card = self.deck1.pop_front().expect("No more cards!");
        let player2_card = self.deck2.pop_front().expect("No more cards!");

        println!("Player 1 plays: {}", player1_card);
        println!("Player 2 plays: {}", player2_card);

        if player1_card > player2_card {
            println!("Player 1 wins the round!");
            self.deck1.push_back(player1_card);
            self.deck1.push_back(player2_card);
        } else {
            println!("Player 2 wins the round!");
            self.deck2.push_back(player2_card);
            self.deck2.push_back(player1_card);
        }
    }

    fn is_winner(&self) -> bool {
        self.deck1.is_empty() || self.deck2.is_empty()
    }

    fn score(&self) -> u64 {
        let deck = if self.deck1.is_empty() { &self.deck2 } else { &self.deck1 };
        let deck_size = deck.len();

        deck.iter().enumerate().map(|(i, &n)| n * (deck_size - i) as u64).sum()
    }
}

type State = (Vec<u64>, Vec<u64>);

struct RecursiveCombat {
    deck1: VecDeque<u64>,
    deck2: VecDeque<u64>,
    seen: HashSet<State>,
    game: u64,
}

impl RecursiveCombat {
    fn new(deck1: VecDeque<u64>, deck2: VecDeque<u64>) -> Self {
        RecursiveCombat { deck1, deck2, seen: HashSet::new(), game: 0 }
    }

    fn play(&mut self) -> u32 {
        let mut round = 0;
        while !self.is_winner() {
            round += 1;

            if self.game == 0 {
                println!("-- Round {} (Game {}) --", round, self.game);
                println!("Player 1's deck: {:?}", self.deck1);
                println!("Player 2's deck: {:?}", self.deck2);
            }

            self.play_round();

            if self.game == 0 {
                println!();
            }
        }

        if self.game == 0 {
            println!();
            println!("== Post-game results ==");
            println!("Player 1's deck: {:?}", self.deck1);
            println!("Player 2's deck: {:?}", self.deck2);
        }

        if self.deck2.is_empty() {
            1
        } else {
            2
        }
    }

    fn play_round(&mut self) {
        let state = self.state();
        let player1_card = self.deck1.pop_front().expect("No more cards!");
        let player2_card = self.deck2.pop_front().expect("No more cards!");

        if self.seen.contains(&state) {
            // Duplicate state - Player 1 wins by default
            self.deck1.push_back(player1_card);
            self.deck1.push_back(player2_card);
            return;
        }

        let winner = if self.deck1.len() as u64 >= player1_card && self.deck2.len() as u64 >= player2_card {
            // Recursive combat!!!
            let deck1: VecDeque<_> = self.deck1.iter().take(player1_card as usize).copied().collect();
            let deck2: VecDeque<_> = self.deck2.iter().take(player2_card as usize).copied().collect();

            let mut combat = RecursiveCombat {
                deck1,
                deck2,
                seen: HashSet::new(),
                game: self.game + 1,
            };

            combat.play()
        } else {
            // Regular combat
            if self.game == 0 {
                println!("Player 1 plays: {}", player1_card);
                println!("Player 2 plays: {}", player2_card);
            }

            if player1_card > player2_card {
                1
            } else {
                2
            }
        };

        if self.game == 0 {
            println!("Player {} wins the round!", winner);
        }

        match winner {
            1 => {
                self.seen.insert(state.clone());
                self.deck1.push_back(player1_card);
                self.deck1.push_back(player2_card);
            },
            2 => {
                self.seen.insert(state.clone());
                self.deck2.push_back(player2_card);
                self.deck2.push_back(player1_card);
            },
            _ => panic!("Unknown winning player {}", winner),
        }
    }

    fn state(&self) -> State {
        let deck1: Vec<_> = self.deck1.iter().copied().collect();
        let deck2: Vec<_> = self.deck2.iter().copied().collect();

        (deck1, deck2)
    }

    fn is_winner(&self) -> bool {
        self.deck1.is_empty() || self.deck2.is_empty()
    }

    fn score(&self) -> u64 {
        let deck = if self.deck1.is_empty() { &self.deck2 } else { &self.deck1 };
        let deck_size = deck.len();

        deck.iter().enumerate().map(|(i, &n)| n * (deck_size - i) as u64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (deck1, deck2) = read_input("sample1.txt");
        let mut combat = Combat::new(deck1.clone(), deck2.clone());
        combat.play();

        assert_eq!(combat.score(), 306);
    }

    #[test]
    fn test_part2() {
        let (deck1, deck2) = read_input("sample1.txt");
        let mut combat = RecursiveCombat::new(deck1.clone(), deck2.clone());
        combat.play();

        assert_eq!(combat.score(), 291);
    }
}

