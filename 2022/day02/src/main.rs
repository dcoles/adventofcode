//! Advent of Code 2022: Day 02
//! https://adventofcode.com/2022/day/02

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = Input::from_file("day02/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut score = 0;

    // "The first column is what your opponent is going to play:
    //  A for Rock, B for Paper, and C for Scissors."
    // The second column, you reason, must be what you should play in response:
    // X for Rock, Y for Paper, and Z for Scissors

    // The score for a single round is the score for the shape you selected
    // (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the
    // outcome of the round (0 if you lost, 3 if the round was a draw,
    // and 6 if you won).
    for (opponent, you) in &input.values {
        score += match (opponent.as_str(), you.as_str()) {
            ("A", "X") => 1 + 3, // Rock vs. Rock (draw)
            ("A", "Y") => 2 + 6, // Rock vs. Paper (win)
            ("A", "Z") => 3 + 0, // Rock vs. Scisors (lose)
            ("B", "X") => 1 + 0, // Paper vs. Rock (lose)
            ("B", "Y") => 2 + 3, // Paper vs. Paper (draw)
            ("B", "Z") => 3 + 6, // Paper vs. Scisors (win)
            ("C", "X") => 1 + 6, // Scisors vs. Rock (win)
            ("C", "Y") => 2 + 0, // Scisors vs. Paper (lose)
            ("C", "Z") => 3 + 3, // Scisors vs. Scisors (draw)
            _ => panic!("Unknown combination: {} {}", opponent, you),
        }
    }

    score
}

fn part2(input: &Input) -> usize {
    let mut score = 0;

    // "Anyway, the second column says how the round needs to end:
    //  X means you need to lose,
    //  Y means you need to end the round in a draw,
    //  and Z means you need to win."
    for (opponent, you) in &input.values {
        score += match (opponent.as_str(), you.as_str()) {
            ("A", "X") => 0 + 3, // Lose = Scisors
            ("A", "Y") => 3 + 1, // Draw = Rock
            ("A", "Z") => 6 + 2, // Win = Paper
            ("B", "X") => 0 + 1, // Lose = Rock
            ("B", "Y") => 3 + 2, // Draw = Paper
            ("B", "Z") => 6 + 3, // Win = Scisors
            ("C", "X") => 0 + 2, // Lose = Paper
            ("C", "Y") => 3 + 3, // Draw = Scisors
            ("C", "Z") => 6 + 1, // Win = Rock
            _ => panic!("Unknown combination: {} {}", opponent, you),
        }
    }

    score
}


#[derive(Debug, Clone)]
struct Input {
    values: Vec<(String, String)>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            let (v1, v2) = line.split_once(' ').unwrap();
            values.push((v1.to_string(), v2.to_string()));
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

        assert_eq!(part1(&input), 15);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 12);
    }
}
