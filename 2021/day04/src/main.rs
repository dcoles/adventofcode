use std::collections::HashSet;
/// Advent of Code 2021: Day 4
/// https://adventofcode.com/2021/day/4

use std::fs;
use std::io;
use std::path::Path;

/// Board size (NxN)
const N: usize = 5;

type Board = Vec<u32>;

fn main() -> io::Result<()> {
    let (boards, draw) = read_input_from_file("day04/input.txt")?;

    println!("Part 1: {}", part1(&boards, &draw));
    println!("Part 2: {}", part2(&boards, &draw));

    Ok(())
}

fn part1(boards: &[Board], draw: &[u32]) -> u32 {
    let mut marked = HashSet::new();
    for &number in draw {
        marked.insert(number);

        for board in boards {
            if check_for_winner(board, &marked) {
                // BINGO!
                return sum_of_unmarked(board, &marked) * number;
            }
        }
    }

    panic!("No winners?!");
}

fn part2(boards: &[Board], draw: &[u32]) -> u32 {
    let mut won = HashSet::new();
    let mut marked = HashSet::new();
    for &number in draw {
        marked.insert(number);

        for (n, board) in boards.iter().enumerate() {
            if check_for_winner(board, &marked) {
                won.insert(n);

                if won.len() == boards.len() {
                    // This is the last board remaining
                    return sum_of_unmarked(board, &marked) * number;
                }
            }
        }
    }
    
    panic!("No winners?!");
}

fn check_for_winner(board: &[u32], marked: &HashSet<u32>) -> bool {
    // Check rows
    for j in 0..N {
        if (0..N).into_iter().all(|i| marked.contains(&board[N * j + i])) {
            return true;
        }
    }

    // Check columns
    for i in 0..N {
        if (0..N).into_iter().all(|j| marked.contains(&board[N * j + i])) {
            return true;
        }
    }

    return false;
}

// Calculate the sum of unmarked numbers
fn sum_of_unmarked(board: &[u32], marked: &HashSet<u32>) -> u32 {
    board.iter().filter(|&x| !marked.contains(x)).sum()
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<(Vec<Board>, Vec<u32>)> {
    let input = fs::read_to_string(path)?;

    let chunks: Vec<_> = input.split("\n\n").collect();

    let draw: Vec<u32> = chunks[0].split(",")
        .map(|c| c.parse().expect("failed to parse number"))
        .collect();

    let mut boards = Vec::new();
    for chunk in &chunks[1..] {
        let board = chunk.split_ascii_whitespace()
            .map(|c| c.parse().expect("failed to parse number"))
            .collect();

        boards.push(board);
    }

    Ok((boards, draw))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let (boards, draw) = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part1(&boards, &draw), 4512);
    }

    #[test]
    fn test_part2() {
        let (boards, draw) = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(part2(&boards, &draw), 1924);
    }
}
