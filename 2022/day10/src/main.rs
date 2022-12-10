//! Advent of Code 2022: Day 10
//! https://adventofcode.com/2022/day/10

use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;

const CRT_WIDTH: usize = 40;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2:");
    for row in part2(&input).chunks(CRT_WIDTH) {
        for c in row {
            // Print double-width for readability
            print!("{}{}", c, c);
        }
        println!();
    }
}

fn part1(input: &Input) -> i32 {
    let mut x = 1;
    let mut cycle = 1;

    let mut values = Vec::new();
    for instruction in &input.values {
        for _ in 0..instruction.cycles() {
            // Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles.
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                // signal strength (the cycle number multiplied by the value of the X register)
                let signal_strength = cycle * x;

                values.push(signal_strength);
            }

            cycle += 1;
        }

        instruction.execute(&mut x);
    }

    // What is the sum of these six signal strengths?
    values.into_iter().sum()
}

fn part2(input: &Input) -> Vec<char> {
    let mut x = 1;
    let mut cycle = 1;

    let mut pixels = Vec::new();
    for instruction in &input.values {
        for _ in 0..instruction.cycles() {
            // CRT columns start from 0
            let col = (cycle - 1) % CRT_WIDTH as i32;

            // `x` represents the middle of a 3-pixel sprite
            pixels.push(if ((x - 1)..=(x + 1)).contains(&col) { '#' } else { '.' });

            cycle += 1;
        }

        instruction.execute(&mut x);
    }

    pixels
}

#[derive(Debug, Clone)]
struct Input {
    values: Vec<Instruction>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut values = Vec::new();
        for line in input.lines() {
            values.push(line.parse().unwrap());
        }

        Ok(Input { values })
    }
}

/// CPU instruction.
#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    /// Number of cycles taken by this instruction.
    fn cycles(self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }

    /// Apply this instruction to a register.
    fn execute(self, reg: &mut i32) {
        match self {
            Self::Noop => (),
            Self::AddX(x) => *reg += x,
        }
    }
}

impl FromStr for Instruction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, value) = s.split_once(' ').unwrap_or((s, ""));

        Ok(match op {
            "noop" => Self::Noop,
            "addx" => {
                let value = value.parse().unwrap();

                Self::AddX(value)
            },
            _ => panic!("Unknown instruction: {}", op),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example2.txt").unwrap();

        assert_eq!(part1(&input), 13140);
    }

    const PART2_IMAGE: [&str; 6] = [
        "##..##..##..##..##..##..##..##..##..##..",
        "###...###...###...###...###...###...###.",
        "####....####....####....####....####....",
        "#####.....#####.....#####.....#####.....",
        "######......######......######......####",
        "#######.......#######.......#######....."
    ];

    #[test]
    fn test_part2() {
        let input = Input::from_file("example2.txt").unwrap();

        for (row, expected) in part2(&input).chunks(CRT_WIDTH).zip(PART2_IMAGE) {
            let row: String = row.iter().collect();

            assert_eq!(row, expected);
        }
    }
}
