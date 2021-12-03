/// Advent of Code 2021: Day 3
/// https://adventofcode.com/2021/day/3

use std::fs;
use std::io;
use std::ops::Index;
use std::path::Path;

fn main() -> io::Result<()> {
    let input = read_input_from_file("day03/input.txt")?;

    // Part 1
    let (gamma, epsilon) = part1(&input);
    println!("Part 1: {}", gamma * epsilon);

    // Part 2
    let (o2_rating, co2_rating) = part2(&input);
    println!("Part 2: {}", o2_rating * co2_rating);

    Ok(())
}

fn read_input_from_file(path: impl AsRef<Path>) -> io::Result<Vec<BitString>> {
    let input = fs::read_to_string(path)?;

    Ok(input.lines().map(BitString::from_str).collect())
}

#[derive(Debug, Clone)]
struct BitString(Vec<u8>);

impl BitString {
    /// Create [`BitString`] from string slice.
    fn from_str(s: &str) -> Self {
        BitString(s.chars().map(|c| {
            match c {
                '0' => 0,
                '1' => 1,
                _ => panic!("Invalid bit value {}", c),
            }
        }).collect())
    }

    /// Value of [`BitString`].
    fn value(&self) -> u32 {
        self.0.iter().fold(0, |acc, &x| (acc << 1) + x as u32)
    }

    /// Length of [`BitString`].
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for BitString {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

fn part1(input: &[BitString]) -> (u32, u32) {
    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..input[0].len() {
        let ones = input.iter().filter(|b| b[i] == 1).count();
        let zeros = input.len() - ones;

        gamma <<= 1;
        epsilon <<= 1;

        if ones > zeros {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    (gamma, epsilon)
}

fn part2(input: &[BitString]) -> (u32, u32) {
    // Oxygen
    let mut list = input.to_vec();
    for i in 0..input[0].len() {
        let ones = list.iter().filter(|b| b[i] == 1).count();
        let zeros = list.len() - ones;

        let most_common = if ones >= zeros { 1 } else { 0 };
        list = list.into_iter().filter(|x| x[i] == most_common).collect();

        if list.len() == 1 {
            break;
        }
    }

    let o2_rating = list[0].value();

    // CO2
    let mut list = input.to_vec();
    for i in 0..input[0].len() {
        let ones = list.iter().filter(|b| b[i] == 1).count();
        let zeros = list.len() - ones;

        let least_common = if ones < zeros { 1 } else { 0 };
        list = list.into_iter().filter(|x| x[i] == least_common).collect();

        if list.len() == 1 {
            break;
        }
    }

    let co2_rating = list[0].value();

    (o2_rating, co2_rating)
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: [&str; 12] = [
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];

    #[test]
    fn test_part1() {
        let input: Vec<_> = INPUT.iter().map(|s| BitString::from_str(s)).collect();

        let (gamma, epsilon) = part1(&input);
        assert_eq!(gamma * epsilon, 198);
    }

    #[test]
    fn test_part2() {
        let input: Vec<_> = INPUT.iter().map(|s| BitString::from_str(s)).collect();

        let (o2_rating, co2_rating) = part2(&input);
        assert_eq!(o2_rating * co2_rating, 230);
    }
}
