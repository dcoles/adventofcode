//! Advent of Code 2021: Day 21
//! https://adventofcode.com/2021/day/21

use std::collections::HashMap;

const PLAYER1: usize = 10;
const PLAYER2: usize = 1;

fn main() {
    // Part 1
    println!("Part 1: {}", part1(PLAYER1, PLAYER2));

    // Part 2
    println!("Part 2: {}", part2(PLAYER1, PLAYER2));
}

fn part1(p1: usize, p2: usize) -> usize {
    let mut die = PracticeDie::new(100);

    let mut player = 0;
    let mut pos = [p1, p2];
    let mut score = [0; 2];
    loop {
        let n = die.roll() + die.roll() + die.roll();
        pos[player] = (pos[player] + n - 1) % 10 + 1;
        score[player] += pos[player];

        if score[player] >= 1000 {
            // Return the score of the losing player multiplied by dice rolls
            return score[(player + 1) % 2] * die.n_rolls;
        }

        player = (player + 1) % 2;
    }
}

fn part2(p1: usize, p2: usize) -> usize {
    // Pre-calculate possible dice results
    let mut rolls: HashMap<usize, usize> = HashMap::new();
    for n in 0..3_usize.pow(3) {
        let x = (n % 3) + 1;
        let y = (n / 3 % 3) + 1;
        let z = (n / 3 / 3 % 3) + 1;

        *rolls.entry(x + y + z).or_default() += 1;
    }

    let mut board = HashMap::new();
    board.insert([(p1, 0), (p2, 0)], 1);

    let mut wins = [0; 2];
    let mut player = 0;
    loop {
        let mut new_board = HashMap::new();
        for (&state, &n_universes) in board.iter() {
            let (pos, score) = state[player];
            for (&roll, &n_copies) in rolls.iter() {
                let new_pos = (pos + roll - 1) % 10 + 1;
                let mut new_state = state.clone();
                new_state[player] = (new_pos, score + new_pos);
                *new_board.entry(new_state).or_default() += n_universes * n_copies;
            }
        }

        let winning: Vec<_> = new_board.keys().copied().filter(|&state | state[player].1 >= 21).collect();
        for win in winning {
            // Stop tracking this state
            wins[player] += new_board.remove(&win).unwrap();
            if new_board.is_empty() {
                // Return the greater number of wins
                return wins.into_iter().max().unwrap()
            }
        }

        board = new_board;
        player = (player + 1) % 2;
    }
}

struct PracticeDie {
    sides: usize,
    n_rolls: usize,
}

impl PracticeDie {
    fn new(sides: usize) -> Self {
        PracticeDie { sides, n_rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        let n = (self.n_rolls % self.sides) + 1;
        self.n_rolls += 1;

        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(4, 8), 739785);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(4, 8), 444356092776315);
    }
}
