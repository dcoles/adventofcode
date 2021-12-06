/// Advent of Code 2021: Day 6
/// https://adventofcode.com/2021/day/6

use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let input = read_input_from_file("day06/input.txt")?;

    // Part 1
    println!("Day 1: {}", simulate_lanternfish(&input, 80));

    // Part 2
    println!("Day 2: {}", simulate_lanternfish(&input, 256));

    Ok(())
}

fn simulate_lanternfish(input: &[usize], n: usize) -> usize {
    // Keep track of internal timers
    let mut population = vec![0; 9];
    for &x in input {
        population[x] += 1;
    }

    // Simulate `n` days
    for _ in 0..n {
        // Current generation of lantern fish
        let x = population[0];

        population[0] = population[1];
        population[1] = population[2];
        population[2] = population[3];
        population[3] = population[4];
        population[4] = population[5];
        population[5] = population[6];
        population[6] = population[7] + x; // Old lanternfish
        population[7] = population[8];
        population[8] = x; // New lanternfish
    }

    population.iter().sum()
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Vec<usize>> {
    let input = fs::read_to_string(path)?;

    Ok(input.trim().split(",").map(|s| s.parse().unwrap()).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(simulate_lanternfish(&input, 80), 5934);
    }

    #[test]
    fn test_part2() {
        let input = read_input_from_file("example1.txt").expect("failed to read input");

        assert_eq!(simulate_lanternfish(&input, 256), 26984457539);
    }
}
