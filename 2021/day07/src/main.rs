//! Advent of Code 2021: Day 7
//! https://adventofcode.com/2021/day/7

use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let input = read_input_from_file("day07/input.txt").expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &[i64]) -> i64 {
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();
    let best_n = (min..=max).min_by_key(|&n| fuel_used1(&input, n)).unwrap();
    
    fuel_used1(&input, best_n)
}

fn part2(input: &[i64]) -> i64 {
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();
    let best_n = (min..=max).min_by_key(|&n| fuel_used2(&input, n)).unwrap();

    fuel_used2(&input, best_n)
}

/// Fuel used to move to position `n`.
fn fuel_used1(input: &[i64], n: i64) -> i64 {
    input.iter().map(|&x| (x - n).abs()).sum()
}

/// Fuel used to move to position `n`.
fn fuel_used2(input: &[i64], n: i64) -> i64 {
    input.iter().map(|&x| {
        let y = (x - n).abs();

        // Triangular number
        (y * (y + 1)) / 2
    }).sum()
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Vec<i64>> {
    let input = fs::read_to_string(path)?;

    Ok(input.trim().split(",").map(|x| x.parse().unwrap()).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: [i64; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 168);
    }
}
